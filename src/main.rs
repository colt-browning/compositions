fn main() {
	let args = std::env::args().skip(1);
	let mut n = 6;
	let mut flat = false;
	let mut reverse = false;
	let mut colex = false;
	let mut minus1 = false;
	let mut by_length = 0;
	for x in args {
		match x.as_ref() {
			"flat" =>	flat = true,
			"reverse"=>	reverse = true,
			"colex" =>	colex = true,
			"-1" =>		minus1 = true,
			"len+" =>	by_length = 1,
			"len-" =>	by_length = -1,
			_ => match x.parse::<usize>() {
				Ok(x) => n = x,
				Err(_) => eprintln!("Illegal argument: {}", x),
			}
		}
	}
	let mut c = vec![vec![vec![Vec::<usize>::new()]]]; // indices: sum, length, № of comopsition, № of its component
	for n in 1..=n {
		let mut cn = vec![];
		if by_length != 0 {
			cn.push(vec![]);
		}
		for len in 1..=(if by_length == 0 {1} else {n}) {
			let mut cnl = vec![];
			let max = if by_length == 0 {n} else {n-len+1};
			let range: Vec<usize> = if reverse {(1..=max).rev().collect()} else {(1..=max).collect()};
			for k in range {
				let prev = &c[n-k][len-1];
				for comp in prev.iter() {
					let mut comp = comp.clone();
					if colex {
						comp.push(k);
					} else {
						comp.insert(0, k);
					}
					cnl.push(comp);
				}
			}
			cn.push(cnl);
		}
		c.push(cn);
	}
	if by_length < 0 {
		for cn in &mut c {
			cn.reverse();
		}
	}
	let mut c: Vec<_> = c.iter_mut().map(
		|cn| -> Vec<_> {cn.iter_mut().fold(vec![], |mut cn, x| {cn.append(x); cn})}
		).collect();
	if minus1 {
		for cn in &mut c {
			for comp in cn {
				for n in comp {
					*n -= 1;
				}
			}
		}
	}
	if flat {
		let mut s = if n == 0 { String::new() } else { format!("{}", c[1][0][0]) };
		for cn in c.iter().skip(2) {
			for comp in cn {
				for n in comp {
					s += &format!(", {}", n);
				}
			}
		}
		println!("{}", s);
		#[cfg(feature = "clipboard-win")]
		if let Err(e) = clipboard_win::set_clipboard_string(&s) {
			eprintln!("{}", e);
		}
	} else {
		for cn in &c {
			println!("{:?}", cn);
		}
	}
}
