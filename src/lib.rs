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
  Принимает мутабельную ссылку на слайс и число N.
  Возвращает мутабельную ссылку на N-ый элемент.
*/
fn mut_element_from_slice(array: &mut [u8], arg: usize) -> &mut u8 {
    &mut array[arg]
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
