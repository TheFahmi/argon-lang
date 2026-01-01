// ============================================
// Cryo Threading Module
// True parallelism using OS threads
// ============================================

#![allow(dead_code)]

use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::collections::HashMap;
use std::time::Duration;

/// Thread-safe value that can be passed between threads
#[derive(Debug, Clone)]
pub enum ThreadValue {
    Null,
    Bool(bool),
    Int(i64),
    String(String),
    Array(Vec<ThreadValue>),
}

impl ThreadValue {
    pub fn to_string_val(&self) -> String {
        match self {
            ThreadValue::Null => "null".to_string(),
            ThreadValue::Bool(b) => b.to_string(),
            ThreadValue::Int(n) => n.to_string(),
            ThreadValue::String(s) => s.clone(),
            ThreadValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string_val()).collect();
                format!("[{}]", items.join(", "))
            }
        }
    }
}

/// Channel for inter-thread communication
pub struct Channel {
    sender: mpsc::Sender<ThreadValue>,
    receiver: Arc<Mutex<mpsc::Receiver<ThreadValue>>>,
}

impl Channel {
    pub fn new() -> (ChannelSender, ChannelReceiver) {
        let (tx, rx) = mpsc::channel();
        (
            ChannelSender { sender: tx },
            ChannelReceiver { receiver: Arc::new(Mutex::new(rx)) }
        )
    }
}

/// Send half of a channel
#[derive(Clone)]
pub struct ChannelSender {
    sender: mpsc::Sender<ThreadValue>,
}

impl ChannelSender {
    pub fn send(&self, value: ThreadValue) -> Result<(), String> {
        self.sender.send(value).map_err(|e| e.to_string())
    }
}

/// Receive half of a channel
pub struct ChannelReceiver {
    receiver: Arc<Mutex<mpsc::Receiver<ThreadValue>>>,
}

impl ChannelReceiver {
    pub fn recv(&self) -> Result<ThreadValue, String> {
        let rx = self.receiver.lock().map_err(|e| e.to_string())?;
        rx.recv().map_err(|e| e.to_string())
    }
    
    pub fn try_recv(&self) -> Option<ThreadValue> {
        let rx = self.receiver.lock().ok()?;
        rx.try_recv().ok()
    }
    
    pub fn recv_timeout(&self, timeout_ms: u64) -> Option<ThreadValue> {
        let rx = self.receiver.lock().ok()?;
        rx.recv_timeout(Duration::from_millis(timeout_ms)).ok()
    }
}

impl Clone for ChannelReceiver {
    fn clone(&self) -> Self {
        ChannelReceiver {
            receiver: Arc::clone(&self.receiver)
        }
    }
}

/// Worker handle for spawned threads
pub struct WorkerHandle {
    pub id: i64,
    handle: Option<JoinHandle<ThreadValue>>,
}

impl WorkerHandle {
    pub fn join(&mut self) -> Result<ThreadValue, String> {
        if let Some(h) = self.handle.take() {
            h.join().map_err(|_| "Thread panicked".to_string())
        } else {
            Err("Worker already joined".to_string())
        }
    }
    
    pub fn is_finished(&self) -> bool {
        self.handle.as_ref().map(|h| h.is_finished()).unwrap_or(true)
    }
}

/// Thread manager - handles all concurrency primitives
pub struct ThreadManager {
    next_worker_id: i64,
    next_channel_id: i64,
    workers: HashMap<i64, WorkerHandle>,
    senders: HashMap<i64, ChannelSender>,
    receivers: HashMap<i64, ChannelReceiver>,
}

impl ThreadManager {
    pub fn new() -> Self {
        ThreadManager {
            next_worker_id: 1,
            next_channel_id: 1,
            workers: HashMap::new(),
            senders: HashMap::new(),
            receivers: HashMap::new(),
        }
    }
    
    /// Create a new unbuffered channel, returns (channel_id)
    pub fn create_channel(&mut self) -> i64 {
        let (sender, receiver) = Channel::new();
        let id = self.next_channel_id;
        self.next_channel_id += 1;
        self.senders.insert(id, sender);
        self.receivers.insert(id, receiver);
        id
    }
    
    /// Create a buffered channel with capacity
    pub fn create_buffered_channel(&mut self, _capacity: usize) -> i64 {
        // Note: mpsc::sync_channel needs different types, simplify to unbuffered for now
        let (sender, receiver) = Channel::new();
        let id = self.next_channel_id;
        self.next_channel_id += 1;
        self.senders.insert(id, sender);
        self.receivers.insert(id, receiver);
        id
    }
    
    /// Send a value to channel
    pub fn channel_send(&self, channel_id: i64, value: ThreadValue) -> bool {
        if let Some(sender) = self.senders.get(&channel_id) {
            sender.send(value).is_ok()
        } else {
            false
        }
    }
    
    /// Receive a value from channel (blocking)
    pub fn channel_recv(&self, channel_id: i64) -> Option<ThreadValue> {
        if let Some(receiver) = self.receivers.get(&channel_id) {
            receiver.recv().ok()
        } else {
            None
        }
    }
    
    /// Try to receive without blocking
    pub fn channel_try_recv(&self, channel_id: i64) -> Option<ThreadValue> {
        if let Some(receiver) = self.receivers.get(&channel_id) {
            receiver.try_recv()
        } else {
            None
        }
    }
    
    /// Receive with timeout
    pub fn channel_recv_timeout(&self, channel_id: i64, timeout_ms: u64) -> Option<ThreadValue> {
        if let Some(receiver) = self.receivers.get(&channel_id) {
            receiver.recv_timeout(timeout_ms)
        } else {
            None
        }
    }
    
    /// Close a channel
    pub fn close_channel(&mut self, channel_id: i64) {
        self.senders.remove(&channel_id);
        // Receiver will get disconnect error
    }
    
    /// Spawn a new worker thread that executes a closure
    pub fn spawn<F>(&mut self, task: F) -> i64 
    where 
        F: FnOnce() -> ThreadValue + Send + 'static
    {
        let id = self.next_worker_id;
        self.next_worker_id += 1;
        
        let handle = thread::spawn(task);
        
        self.workers.insert(id, WorkerHandle {
            id,
            handle: Some(handle),
        });
        
        id
    }
    
    /// Spawn with a simple value computation
    pub fn spawn_compute(&mut self, value: i64, operation: &str) -> i64 {
        let op = operation.to_string();
        let id = self.next_worker_id;
        self.next_worker_id += 1;
        
        let handle = thread::spawn(move || {
            match op.as_str() {
                "double" => ThreadValue::Int(value * 2),
                "square" => ThreadValue::Int(value * value),
                "factorial" => {
                    let mut result = 1i64;
                    for i in 1..=value {
                        result *= i;
                    }
                    ThreadValue::Int(result)
                }
                "fib" => {
                    if value < 2 {
                        ThreadValue::Int(value)
                    } else {
                        let mut a = 0i64;
                        let mut b = 1i64;
                        for _ in 2..=value {
                            let temp = a + b;
                            a = b;
                            b = temp;
                        }
                        ThreadValue::Int(b)
                    }
                }
                "sleep" => {
                    thread::sleep(Duration::from_millis(value as u64));
                    ThreadValue::Int(value)
                }
                _ => ThreadValue::Int(value)
            }
        });
        
        self.workers.insert(id, WorkerHandle {
            id,
            handle: Some(handle),
        });
        
        id
    }
    
    /// Join a worker (wait for completion)
    pub fn join_worker(&mut self, worker_id: i64) -> Option<ThreadValue> {
        if let Some(mut worker) = self.workers.remove(&worker_id) {
            worker.join().ok()
        } else {
            None
        }
    }
    
    /// Check if worker is finished
    pub fn is_worker_finished(&self, worker_id: i64) -> bool {
        if let Some(worker) = self.workers.get(&worker_id) {
            worker.is_finished()
        } else {
            true
        }
    }
    
    /// Get number of active workers
    pub fn active_workers(&self) -> usize {
        self.workers.values().filter(|w| !w.is_finished()).count()
    }
    
    /// Join all workers
    pub fn join_all(&mut self) -> Vec<ThreadValue> {
        let ids: Vec<i64> = self.workers.keys().cloned().collect();
        ids.into_iter()
            .filter_map(|id| self.join_worker(id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_channel() {
        let mut tm = ThreadManager::new();
        let ch_id = tm.create_channel();
        
        // Send in separate thread
        let sender = tm.senders.get(&ch_id).unwrap().clone();
        thread::spawn(move || {
            sender.send(ThreadValue::Int(42)).unwrap();
        });
        
        // Receive
        let result = tm.channel_recv(ch_id);
        assert!(matches!(result, Some(ThreadValue::Int(42))));
    }
    
    #[test]
    fn test_spawn_compute() {
        let mut tm = ThreadManager::new();
        
        let w1 = tm.spawn_compute(5, "factorial");
        let w2 = tm.spawn_compute(10, "fib");
        
        let r1 = tm.join_worker(w1);
        let r2 = tm.join_worker(w2);
        
        assert!(matches!(r1, Some(ThreadValue::Int(120)))); // 5!
        assert!(matches!(r2, Some(ThreadValue::Int(55))));  // fib(10)
    }
}
