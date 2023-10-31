use grambulate::grambulate;

#[test]
fn test_grambulate() -> Result<(), String> {
    assert_eq!(grambulate(1, 2)?, 11);
    assert_eq!(grambulate(1, 9)?, 25);
    assert_eq!(grambulate(1, 7)?, 21);
    assert_eq!(grambulate(7, 1)?, 3);
    assert_eq!(grambulate(9, 5)?, 37);
    assert_eq!(grambulate(114, 116)?, 118);

    assert_eq!(grambulate(120842908, 765255103)?, 2562110898);
    assert_eq!(grambulate(283311628, 982590465)?, 3030985010);
    assert_eq!(grambulate(841055110, 210288268)?, 1373718672);
    assert_eq!(grambulate(435594887, 106289809)?, 1721403763);
    assert_eq!(grambulate(239517836, 742207561)?, 4894885652);
    assert_eq!(grambulate(128398453, 764847139)?, 2794172097);
    assert_eq!(grambulate(751271524, 945230955)?, 4979352842);

    // precision
    assert_eq!(
        grambulate(9223372036854775807, 9223372036854775808)?,
        9223372036854775809
    );

    Ok(())
}
