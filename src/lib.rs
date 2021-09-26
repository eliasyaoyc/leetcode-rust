use std::rc::Rc;
use std::cell::RefCell;

mod _102_level_order;
mod _103_zigzag_level_order;
mod _107_level_order_bottom;
mod _111_min_depth;
mod _113_path_sum;
mod _129_sum_numbers;
mod _145_postorder_traversal;
mod _331_is_valid_serialization;
mod _404_sum_of_left_levels;
mod _449_codec;
mod _199_right_side_view;
mod _501_find_mode;
mod _513_find_bottom_left_value;
mod _530_get_minimum_difference;
mod _508_find_frequent_tree_sum;
mod _515_largest_values;
mod _572_is_subtree;
mod _563_find_tilt;
mod _617_merge_trees;
mod _606_tree2str;
mod _637_average_of_levels;
mod _623_add_one_row;
mod _653_find_target;
mod _671_find_second_minimum_value;
mod _655_print_tree;
mod _669_trim_bst;
mod _662_width_of_binary_tree;
mod _814_prune_tree;
mod _1325_remove_leaf_nodes;
mod _1110_del_nodes;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}