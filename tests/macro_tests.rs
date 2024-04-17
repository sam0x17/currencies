use currencies::amt;
use currencies::amt_checked;
use currencies::currency::*;
use currencies::safety::Checked;
use currencies::Amount;

#[test]
fn test_amt_usd_unchecked() {
    let amount = amt!(USD, "$3.24");
    assert_eq!(format!("{}", amount), "$3.24");
}

#[test]
fn test_amt_usd_checked() {
    let amount: Amount<USD, Checked> = amt_checked!(USD, "$3.24");
    assert_eq!(format!("{}", amount), "$3.24");
}
