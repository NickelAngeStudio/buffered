/*
 * @file tampon/compare.rs
 *
 * @module tampon
 *
 * @brief Contain function that compare 2 buffers.
 * 
 * @details
 * Contain function that compare 2 buffers.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-07-01 - Happy Canada Day
 *
 * @version
 * 1.0 : 2022-07-01 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

/// ##### Compare 2 buffers and return the absolute difference.
/// Compare 2 buffers and return the absolute difference.
/// 
/// If both buffers are identical in size AND content, this function will return 0.
/// # Example(s)
/// ```
/// // Import tampon function
/// use tampon::compare_buffers;
/// 
/// let b1:Vec<u8> = vec![5;5];
/// let b2:Vec<u8> = vec![5;5];
/// 
/// // Both buffer should be equal and return 0
/// assert!(tampon::compare_buffers(&b1,&b2)==0);
/// 
/// let b3:Vec<u8> = vec![4;4];
/// 
/// // Buffers should be different
/// assert!(tampon::compare_buffers(&b1,&b3)>0)
/// ```
/// # Argument(s)
/// * `b1` - First `Vec<u8>` buffer reference to compare.
/// * `b2` - Second `Vec<u8>` buffer reference to compare.
/// # Return
/// Absolute difference between both buffers. Identical in size and content will return 0.
pub fn compare_buffers(b1 : &Vec<u8>,  b2 : &Vec<u8>) -> usize {
        
    // Difference is initialize with the absolute difference in length
    let mut _diff: usize = if b1.len() > b2.len() {
        b1.len() - b2.len()
    } else {
        b2.len() - b1.len()
    };

    // Size will be the lowest of both sizes
    let size = std::cmp::min(b1.len(), b2.len());

    // Calculate the difference in size and take the lowest size of both buffers.
    if b1.len() > b2.len() {
        _diff = b1.len() - b2.len();
    } else {
        _diff = b2.len() - b1.len();
    }
    
    // Calculate the absolute difference of b1, b2
    for i in 0..size {

        if b1[i] > b2[i] {
            _diff = _diff + (b1[i] - b2[i]) as usize;
        } else {
            _diff = _diff + (b2[i] - b1[i]) as usize;
        }
    }
     
    // Return the difference
    _diff
}