### ch09 [Error Handling] (https://doc.rust-lang.org/book/ch09-00-error-handling.html "Error Handling")

* Using panic! Backtrace

      `RUST_BACKTRACE=1 cargo run`
      
* Recoverable Errors with Result

     * matching on different Errors
     * shortcut: `unwrap` `expect` `unwrap_or_else`
     
* Propagating Errors

     * return Result
     * shortcut `?` operator
         > There is a difference between what the match expression and the ? operator do: error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another. When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. As long as each error type implements the from function to define how to convert itself to the returned error type, the ? operator takes care of the conversion automatically.
         
