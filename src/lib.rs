#![allow(dead_code)]
/*
 Принимает мутабельную ссылку на кортеж и bool значение.
 Если false, возвращает мутабельную ссылку на первый элемент кортежа.
 Если true, возвращает мутабельную ссылку на второй элемент кортежа.
*/
pub fn element_from_unit(a: &mut (i32, i32), first: bool) -> &i32 {
    let (x, y) = a;
    if first {
        y
    } else {
        x
    }
}

/*
 Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
*/
fn mut_element_from_slice(array: &mut [u8], n: usize) -> &mut u8 {
    &mut array[n]
}

/*
  Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
*/
fn element_from_end_of_slice(array: &[u8], n: usize) -> &u8 {
    let len = array.len();
    &array[len - 1 - n]
}

/*
  Принимает слайс и число N. Возвращает два слайса с элементами:
   с нулевого по N-1;
   с N-го по последний;
*/
fn two_slices_from_one_slice(array: &[u8], n: usize) -> (&[u8], &[u8]) {
    (&array[..n], &array[n..])
}

/*
  Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
*/
fn slice_array(array: &[u8]) -> [&[u8]; 4] {
    let len = array.len();
    let i = len / 4;
    [&array[..i], &array[i..2 * i], &array[2 * i..3 * i], &array[3 * i..]]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_array() {
        assert_eq!([&[1], &[2], &[3], &[4]], slice_array(&[1, 2, 3, 4]));
        assert_eq!([&[1, 2], &[3, 4], &[5, 6], &[7, 8]], slice_array(&[1, 2, 3, 4, 5, 6, 7, 8]));
    }

    #[test]
    fn test_unit_bool() {
        let x = 1;
        let y = 2;
        assert_eq!(&x, element_from_unit(&mut (x, y), false));
        assert_eq!(&y, element_from_unit(&mut (x, y), true));
    }

    #[test]
    fn test_mut_element_from_slice() {
        assert_eq!(&1, mut_element_from_slice(&mut [1, 2, 3], 0));
        assert_eq!(&77, mut_element_from_slice(&mut [0, 77, 0], 1));
    }

    #[test]
    fn test_element_from_end_of_slice() {
        let array = &[1, 2, 3, 4, 5];
        assert_eq!(&5, element_from_end_of_slice(array, 0));
        assert_eq!(&1, element_from_end_of_slice(array, 4));
        assert_eq!(&3, element_from_end_of_slice(array, 2));
    }

    #[test]
    fn test_two_elements_from_slice() {
        let array = &[1, 2, 3, 4, 5];
        let (a, b) = two_slices_from_one_slice(array, 2);
        assert_eq!(&[1, 2], a);
        assert_eq!(&[3, 4, 5], b);
    }
}
