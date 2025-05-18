

/*
REDDIT POST:
--------------------------------------------------------------------------
u/Atro-2004
  What is the difference between constants, and immutable variables in Rust?
--------------------------------------------------------------------------
u/IAm_A_Complete_Idiot:
    constants must be known at compile time. e.g. the size of an array must 
    be constant. A const fn must be evaluatable at compile time if it's 
    arguments are.
    
    Immutable variables don't have this restriction, they only state you 
    can't get a unique (mutable) reference to the variable.
    |
    u/vklein52
        A quick example (obviously not complete) is that I’d use a constant 
        for the number of threads I’d use to split up a task (fixed size, 
        always the same across executions and during a single execution). 
        I’d use an immutable vector to contain the arguments to the executable 
        (variable size, different each execution, not changing over the course 
        of a single execution). 
*/

pub fn run() {
    const NUM_THREADS: u8 = 3;
    println!("NUM THREADS: {}", NUM_THREADS);
}

