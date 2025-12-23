// Hotspot Profiler for detecting frequently executed code

use std::collections::{HashMap, HashSet};
use crate::types::FunctionId;

/// Hotspot profiler for tracking function execution frequency
#[derive(Debug, Clone)]
pub struct HotspotProfiler {
    /// Execution count for each function
    execution_counts: HashMap<FunctionId, usize>,
    /// Threshold for marking a function as hot
    hotspot_threshold: usize,
    /// Set of functions marked as hot
    hot_functions: HashSet<FunctionId>,
}

impl HotspotProfiler {
    /// Create a new profiler with the given threshold
    pub fn new(threshold: usize) -> Self {
        Self {
            execution_counts: HashMap::new(),
            hotspot_threshold: threshold,
            hot_functions: HashSet::new(),
        }
    }
    
    /// Create a profiler with default threshold (100)
    pub fn default_threshold() -> Self {
        Self::new(100)
    }
    
    /// Record an execution of a function
    pub fn record_execution(&mut self, func_id: FunctionId) {
        let count = self.execution_counts.entry(func_id).or_insert(0);
        *count += 1;
        
        // Check if function should be marked as hot
        if *count >= self.hotspot_threshold && !self.hot_functions.contains(&func_id) {
            self.mark_hot(func_id);
        }
    }
    
    /// Check if a function is marked as hot
    pub fn is_hot(&self, func_id: FunctionId) -> bool {
        self.hot_functions.contains(&func_id)
    }
    
    /// Mark a function as hot
    pub fn mark_hot(&mut self, func_id: FunctionId) {
        self.hot_functions.insert(func_id);
    }
    
    /// Unmark a function as hot (used after deoptimization)
    pub fn unmark_hot(&mut self, func_id: FunctionId) {
        self.hot_functions.remove(&func_id);
    }
    
    /// Get the execution count for a function
    pub fn get_count(&self, func_id: FunctionId) -> usize {
        self.execution_counts.get(&func_id).copied().unwrap_or(0)
    }
    
    /// Get the hotspot threshold
    pub fn threshold(&self) -> usize {
        self.hotspot_threshold
    }
    
    /// Reset all execution counts
    pub fn reset(&mut self) {
        self.execution_counts.clear();
        self.hot_functions.clear();
    }
}

impl Default for HotspotProfiler {
    fn default() -> Self {
        Self::default_threshold()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_profiler_creation() {
        let profiler = HotspotProfiler::new(50);
        assert_eq!(profiler.threshold(), 50);
    }
    
    #[test]
    fn test_record_execution() {
        let mut profiler = HotspotProfiler::new(10);
        
        profiler.record_execution(0);
        assert_eq!(profiler.get_count(0), 1);
        
        profiler.record_execution(0);
        assert_eq!(profiler.get_count(0), 2);
    }
    
    #[test]
    fn test_hotspot_detection() {
        let mut profiler = HotspotProfiler::new(3);
        
        assert!(!profiler.is_hot(0));
        
        profiler.record_execution(0);
        profiler.record_execution(0);
        assert!(!profiler.is_hot(0));
        
        profiler.record_execution(0);
        assert!(profiler.is_hot(0));
    }
    
    #[test]
    fn test_mark_hot() {
        let mut profiler = HotspotProfiler::new(100);
        
        assert!(!profiler.is_hot(0));
        profiler.mark_hot(0);
        assert!(profiler.is_hot(0));
    }
    
    #[test]
    fn test_unmark_hot() {
        let mut profiler = HotspotProfiler::new(100);
        
        profiler.mark_hot(0);
        assert!(profiler.is_hot(0));
        
        profiler.unmark_hot(0);
        assert!(!profiler.is_hot(0));
    }
    
    #[test]
    fn test_multiple_functions() {
        let mut profiler = HotspotProfiler::new(2);
        
        profiler.record_execution(0);
        profiler.record_execution(1);
        profiler.record_execution(0);
        
        assert!(profiler.is_hot(0));
        assert!(!profiler.is_hot(1));
        
        profiler.record_execution(1);
        assert!(profiler.is_hot(1));
    }
    
    #[test]
    fn test_reset() {
        let mut profiler = HotspotProfiler::new(2);
        
        profiler.record_execution(0);
        profiler.record_execution(0);
        assert!(profiler.is_hot(0));
        
        profiler.reset();
        assert_eq!(profiler.get_count(0), 0);
        assert!(!profiler.is_hot(0));
    }
}
