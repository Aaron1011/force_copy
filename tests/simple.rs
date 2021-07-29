use force_copy::ForceCopy;

#[derive(Debug)]
pub struct NotCopy(pub u8);

#[derive(Copy, Clone)]
pub struct Wrapper {
    pub val: ForceCopy<NotCopy>
}

#[test]
fn test_notcopy() {
    let val = unsafe { ForceCopy::new(NotCopy(25)) };
    let first = val;
    let second = val;
    assert_eq!(25, unsafe { first.into_inner().0 });
	assert_eq!(25, unsafe { second.into_inner().0 });
}
