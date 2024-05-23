fn main() { 
}

pub fn convert(s: String, num_rows: i32) -> String {
  let mut output_string: Vec<u8> = Vec::new();
  let mut num_columns = 0;
  let mut column_iterator = 0;
  let s_len = s.len();

  while column_iterator < s_len {
    let mut i = 0;
    while i < num_rows {
      column_iterator += 1;
      i += 1;
    }

    num_columns += 1;

    if num_rows > 2 {
      let rows_up = num_rows - 2;
      let mut i = 0;
      while i < rows_up {
        if column_iterator < s_len {
          num_columns += 1;
          column_iterator += 1;
        }
        i += 1;
      }
    }
  }

  // let mut matrix: Vec<Vec<u8>> = Vec::new();
  let mut matrix: Vec<Vec<u8>> = vec![vec![b' '; num_columns]; num_rows as usize];
  let mut column = 0;
  let mut iterator = 0;
  while iterator < s_len {
    //zig down
    let mut i = 0;
    while i < num_rows {
      if iterator < s_len {
        matrix[i as usize][column] = s.as_bytes()[iterator];
        iterator += 1;
      }
      i += 1;
    }

    column += 1;

    //zig back up
    if num_rows > 2 {
      let rows_up = num_rows - 2;
      let mut i = 0;
      while i < rows_up {
        if iterator < s_len {
          matrix[(num_rows - 2 - i) as usize][column] = s.as_bytes()[iterator];
          iterator += 1;
          column += 1;
        }
        i += 1;
      }
    }
  }

  let mut i: usize = 0; while i < num_rows as usize {
    let mut j: usize = 0; while j < num_columns {
      if matrix[i][j] != b' ' {
        output_string.push(matrix[i][j]);
      }
      j += 1;
    }
    i += 1;
  }
  String::from_utf8(output_string).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asserts() {
      assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI".to_string());
    }
}