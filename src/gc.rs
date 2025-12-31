// Argon Garbage Collector Module
// Mark-and-Sweep GC for managing heap-allocated objects

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
    Ref(ObjectId),  // Reference to heap object
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
    roots: Vec<ObjectId>,  // Root set (stack references)
    threshold: usize,      // Collection threshold
    allocated: usize,      // Current allocation count
}

impl GarbageCollector {
    pub fn new() -> Self {
        GarbageCollector {
            heap: HashMap::new(),
            next_id: 1,
            roots: Vec::new(),
            threshold: 1000,  // Collect after 1000 allocations
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
        
        // Check if we should collect
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
    
    /// Get mutable access to array
    pub fn get_array_mut(&self, id: ObjectId) -> Option<std::cell::RefMut<Vec<GcValue>>> {
        self.heap.get(&id).and_then(|h| {
            let header = h.borrow_mut();
            if matches!(&header.data, GcObject::Array(_)) {
                Some(std::cell::RefMut::map(h.borrow_mut(), |h| {
                    if let GcObject::Array(arr) = &mut h.data {
                        arr
                    } else {
                        unreachable!()
                    }
                }))
            } else {
                None
            }
        })
    }
    
    /// Add a root reference (called when value enters stack)
    pub fn add_root(&mut self, id: ObjectId) {
        if !self.roots.contains(&id) {
            self.roots.push(id);
        }
    }
    
    /// Remove a root reference (called when value leaves stack)
    pub fn remove_root(&mut self, id: ObjectId) {
        self.roots.retain(|&r| r != id);
    }
    
    /// Clear all roots (e.g., at scope exit)
    pub fn clear_roots(&mut self) {
        self.roots.clear();
    }
    
    /// Run garbage collection (Mark-and-Sweep)
    pub fn collect(&mut self) {
        // Phase 1: Mark
        self.mark_phase();
        
        // Phase 2: Sweep
        self.sweep_phase();
        
        // Reset allocation counter
        self.allocated = 0;
    }
    
    /// Mark phase: trace from roots
    fn mark_phase(&mut self) {
        // Reset all marks
        for header in self.heap.values() {
            header.borrow_mut().marked = false;
        }
        
        // Mark from roots
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
                return; // Already marked, avoid cycles
            }
            h.marked = true;
            
            // Mark children
            match &h.data {
                GcObject::Array(arr) => {
                    let refs: Vec<ObjectId> = arr.iter()
                        .filter_map(|v| if let GcValue::Ref(r) = v { Some(*r) } else { None })
                        .collect();
                    drop(h); // Release borrow before recursive call
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
        
        // Allocate without adding to roots
        let id1 = gc.alloc_array(vec![]);
        let id2 = gc.alloc_array(vec![]);
        
        // Add only id1 to roots
        gc.add_root(id1);
        
        // Collect
        gc.collect();
        
        // id1 should survive, id2 should be collected
        assert!(gc.get(id1).is_some());
        assert!(gc.get(id2).is_none());
    }
}
