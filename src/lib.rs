/*
It recognizes as headers:

* multiple single-line comments ``//`` with possibly blank
  inter-lines, that appear as first non blank lines (except ``package``)
  or just after lines considered as headers too.

* multiple lines in a multi-line comment ``/* … */`` that appear as first
  non blank lines (except ``package``)
  or just after lines considered as headers too.

* any combination of the previous two kinds of headers appearing
  continuous, until the first non-blank non-package line.
  */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
