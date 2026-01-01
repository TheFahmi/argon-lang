// Cryo Garbage Collector Module
// Mark-and-Sweep GC for managing heap-allocated objects

#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;

/// Object ID type
pub type ObjectId = usize;

/// GC-managed object types
#[derive(Debug, Clone)]
pub enum GcObject {
    Array(Vec<GcValue>),
    Struct(String, HashMap<String, GcValue>),
}

/// Value that can reference GC objects
#[derive(Debug, Clone)]
pub enum GcValue {
    Null,
    Bool(bool),
    Int(i64),
    String(String),
    Ref(ObjectId),
}

/// Object header for GC tracking
#[derive(Debug)]
struct ObjectHeader {
    marked: bool,
    data: GcObject,
}

/// The Garbage Collector
pub struct GarbageCollector {
    heap: HashMap<ObjectId, RefCell<ObjectHeader>>,
    next_id: ObjectId,
    roots: Vec<ObjectId>,
    threshold: usize,
    allocated: usize,
}

impl GarbageCollector {
    pub fn new() -> Self {
        GarbageCollector {
            heap: HashMap::new(),
            next_id: 1,
            roots: Vec::new(),
            threshold: 1000,
            allocated: 0,
        }
    }
    
    /// Allocate a new object on the heap
    pub fn alloc(&mut self, obj: GcObject) -> ObjectId {
        let id = self.next_id;
        self.next_id += 1;
        
        self.heap.insert(id, RefCell::new(ObjectHeader {
            marked: false,
            data: obj,
        }));
        
        self.allocated += 1;
        
        if self.allocated >= self.threshold {
            self.collect();
        }
        
        id
    }
    
    /// Allocate an array
    pub fn alloc_array(&mut self, items: Vec<GcValue>) -> ObjectId {
        self.alloc(GcObject::Array(items))
    }
    
    /// Allocate a struct
    pub fn alloc_struct(&mut self, name: String, fields: HashMap<String, GcValue>) -> ObjectId {
        self.alloc(GcObject::Struct(name, fields))
    }
    
    /// Get an object by ID
    pub fn get(&self, id: ObjectId) -> Option<GcObject> {
        self.heap.get(&id).map(|h| h.borrow().data.clone())
    }
    
    /// Add a root reference
    pub fn add_root(&mut self, id: ObjectId) {
        if !self.roots.contains(&id) {
            self.roots.push(id);
        }
    }
    
    /// Remove a root reference
    pub fn remove_root(&mut self, id: ObjectId) {
        self.roots.retain(|&r| r != id);
    }
    
    /// Run garbage collection (Mark-and-Sweep)
    pub fn collect(&mut self) {
        self.mark_phase();
        self.sweep_phase();
        self.allocated = 0;
    }
    
    /// Mark phase: trace from roots
    fn mark_phase(&mut self) {
        for header in self.heap.values() {
            header.borrow_mut().marked = false;
        }
        
        let roots = self.roots.clone();
        for root in roots {
            self.mark(root);
        }
    }
    
    /// Mark an object and its children
    fn mark(&self, id: ObjectId) {
        if let Some(header) = self.heap.get(&id) {
            let mut h = header.borrow_mut();
            if h.marked {
                return;
            }
            h.marked = true;
            
            match &h.data {
                GcObject::Array(arr) => {
                    let refs: Vec<ObjectId> = arr.iter()
                        .filter_map(|v| if let GcValue::Ref(r) = v { Some(*r) } else { None })
                        .collect();
                    drop(h);
                    for child in refs {
                        self.mark(child);
                    }
                }
                GcObject::Struct(_, fields) => {
                    let refs: Vec<ObjectId> = fields.values()
                        .filter_map(|v| if let GcValue::Ref(r) = v { Some(*r) } else { None })
                        .collect();
                    drop(h);
                    for child in refs {
                        self.mark(child);
                    }
                }
            }
        }
    }
    
    /// Sweep phase: free unmarked objects
    fn sweep_phase(&mut self) {
        let dead: Vec<ObjectId> = self.heap.iter()
            .filter(|(_, h)| !h.borrow().marked)
            .map(|(id, _)| *id)
            .collect();
        
        for id in dead {
            self.heap.remove(&id);
        }
    }
    
    /// Get heap statistics
    pub fn stats(&self) -> (usize, usize) {
        (self.heap.len(), self.allocated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gc_alloc() {
        let mut gc = GarbageCollector::new();
        let id = gc.alloc_array(vec![GcValue::Int(1), GcValue::Int(2)]);
        assert!(gc.get(id).is_some());
    }
    
    #[test]
    fn test_gc_collect() {
        let mut gc = GarbageCollector::new();
        let id1 = gc.alloc_array(vec![]);
        let id2 = gc.alloc_array(vec![]);
        gc.add_root(id1);
        gc.collect();
        assert!(gc.get(id1).is_some());
        assert!(gc.get(id2).is_none());
    }
}
