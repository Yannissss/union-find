use union_find::{PinnedRepr, Repr};

#[test]
fn test_self_find() {
    let mut x: PinnedRepr<()> = Repr::new(());
    assert!(x.is_root())
}

#[test]
fn test_separate() {
    let mut x: PinnedRepr<u32> = Repr::new(1);
    let mut y: PinnedRepr<u32> = Repr::new(2);
    assert_ne!(x.find(), y.find());
}

#[test]
fn test_union() {
    let mut x: PinnedRepr<u32> = Repr::new(1);
    let mut y: PinnedRepr<u32> = Repr::new(2);
    x.union(&mut y);
    assert_eq!(x.find(), y.find());
}

#[test]
fn test_partition() {
    let v = vec![10, 2, 7, 8, 19, 22, 3];

    let mut xs = Vec::new();
    for x in v {
        xs.push(Repr::new(x));
    }

    let mut odd = Repr::new(0);
    let mut even = Repr::new(0);

    for x in xs.iter_mut() {
        if x.tag % 2 == 0 {
            even.union(x);
        } else {
            odd.union(x);
        }
    }

    let odd_repr = odd.find();
    let even_repr = even.find();
    let odds = xs
        .iter_mut()
        .filter_map(|x| {
            if x.find() == odd_repr {
                Some(x.tag)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let evens = xs
        .iter_mut()
        .filter_map(|x| {
            if x.find() == even_repr {
                Some(x.tag)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(&odds[..], &[7, 19, 3]);
    assert_eq!(&evens[..], &[10, 2, 8, 22]);
}
