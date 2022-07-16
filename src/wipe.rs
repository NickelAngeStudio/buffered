/*
 * @file tampon/wipe.rs
 *
 * @module tampon
 *
 * @brief Contain function to wipe a buffer.
 * 
 * @details
 * Contain function to wipe a buffer.
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

 /// ##### Wipe a buffer, overwriting content with zeroes.
 /// Wipe a sensible buffer to prevent [`cold boot attack`](https://en.wikipedia.org/wiki/Cold_boot_attack) for greater security.
 /// 
 /// # Argument(s)
 /// * `buffer` - Mutable reference to vector of [`u8`] to wipe.
 /// 
 /// # Warning(s)
 /// <b>It goes without saying that it can't be reversed.</b>
 /// 
 /// # Example(s)
 /// ```
 /// // Import tampon crate.
 /// use tampon::wipe_buffer;
 /// 
 /// // Create a u8 array
 /// let mut buffer : &mut Vec<u8> = &mut vec![80, 76, 90, 87, 73, 80, 69, 77, 69];
 /// 
 /// // Print current buffer
 /// println!("Buffer = {:?}", buffer);
 /// 
 /// // Wipe buffer
 /// tampon::wipe_buffer(&mut buffer);
 /// 
 /// // Print wiped buffer
 /// println!("Buffer = {:?}", buffer);
 /// ```
 pub fn wipe_buffer(buffer : &mut Vec<u8>){

    for elem in buffer.iter_mut() {
        *elem = 0;
    }

 }