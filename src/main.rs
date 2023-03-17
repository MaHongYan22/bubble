fn main() {
    let  a=&mut vec![1,9,6,7,2];
    let mut strT = vec![
            String::from("Bob"),
            String::from("David"),
            
            String::from("Alice"),
            String::from("Carol"),
        ];
    bubble_sort2(&mut strT);

    bubble_sort2(a);
    println!("array is {:?}",a);
    println!("array is {:?}",strT);
}
fn bubble_sort(nums: &mut Vec<usize>) {
  let length=nums.len();
  for i in 0..length{
    for j in 0..length-i-1{
        if nums[j] >nums[j+1] {
            nums.swap(j, j+1);
    }
  }
}
}
pub fn bubble_sort2<T: PartialOrd>(array: &mut [T]) {

    let size = array.len();
    // 单个元素集合或者空集合 直接返回 不需要排序
    if size <= 1 {
        return;
    }

    // 对所有元素进行重复，除了最后一个
    for i in 0..(size-1) {
        // 记录是否发生过交换
        let mut swapped = false;
        
        for j in 1..(size - i) {
            // 比较相邻的元素 如果第一个比第二个大，就交换他们两个。
            if array[j - 1]  > array[j] {
                // 切片交换两个元素 
                array.swap(j - 1, j);
                swapped = true;
            }
        }

        // 如果未发生交换，说明已经排好序，跳出后续的冒泡
        if !swapped {
            break;
        }
    }
}