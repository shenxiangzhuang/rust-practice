/// reference: https://google.github.io/comprehensive-rust/generics/exercise.html

trait LessThan {
    /// Return true if self is less than other.
    fn less_than(&self, other: &Self) -> bool;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Citation {
    author: &'static str,
    year: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]  // without Copy Trait here
struct CitationWithoutCopy {
    author: &'static str,
    year: u32,
}

impl LessThan for Citation {
    fn less_than(&self, other: &Self) -> bool {
        if self.author < other.author {
            true
        } else if self.author > other.author {
            false
        } else {
            self.year < other.year
        }
    }
}


impl LessThan for CitationWithoutCopy {
    fn less_than(&self, other: &Self) -> bool {
        if self.author < other.author {
            true
        } else if self.author > other.author {
            false
        } else {
            self.year < other.year
        }
    }
}

fn min<T: LessThan>(lhs: T, rhs: T) -> T {
    if lhs.less_than(&rhs) {  // 注意这里应该用&rhs, 否则会被move
        lhs
    } else {
        rhs
    }
}

fn min_by_ref<'a, T: LessThan>(lhs: &'a T, rhs: &'a T) -> &'a T {
    if lhs.less_than(rhs) {
        lhs
    } else {
        rhs
    }
}


pub fn run_generic_example() {
    let cit1 = Citation { author: "Shapiro", year: 2011 };
    let cit2 = Citation { author: "Baumann", year: 2010 };
    let cit3 = Citation { author: "Baumann", year: 2019 };
    debug_assert_eq!(min(cit1, cit2), cit2);
    debug_assert_eq!(min(cit2, cit3), cit2);
    debug_assert_eq!(min(cit1, cit3), cit3);

    let cit11 = CitationWithoutCopy { author: "Shapiro", year: 2011 };
    let cit22 = CitationWithoutCopy { author: "Baumann", year: 2010 };
    let cit33 = CitationWithoutCopy{ author: "Baumann", year: 2019 };
    debug_assert_eq!(min_by_ref(&cit11, &cit22), &cit22);
    debug_assert_eq!(min_by_ref(&cit22, &cit33), &cit22);
    debug_assert_eq!(min_by_ref(&cit11, &cit33), &cit33);
    println!("Finished!")
}