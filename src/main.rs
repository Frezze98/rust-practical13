// Функція, яка рахує мінімальну кількість переносу вантажу, щоб на всіх кораблях був однаковий вантаж
fn count_permutation(shipments: &Vec<u32>) -> i32 {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return -1;
    }

    let target = total / n;
    let mut excess = Vec::new();
    let mut deficit = Vec::new();
    let mut moves: i32 = 0;

    for &shipment in shipments {
        if shipment > target {
            excess.push(shipment - target);
        } else if shipment < target {
            deficit.push(target - shipment);
        }
    }

    while !excess.is_empty() && !deficit.is_empty() {
        let e = excess.pop().unwrap();
        let d = deficit.pop().unwrap();

        let min_move = if e < d { e } else { d };

        moves += min_move as i32;
        if e > d {
            excess.push(e - d);
        } else if d > e {
            deficit.push(d - e);
        }
    }

    moves
}

// Функція генерації Vec<u32>, які можуть бути розподілені однаково
fn gen_shipments(n: usize) -> Vec<u32> {
    let base: u32 = 100; // або будь-яке інше число
    let mut shipments = vec![base; n];

    // Додаємо випадковий вантаж, щоб загальна кількість вантажу була кратною n
    let total: u32 = base * n as u32;
    for i in 0..(total % n as u32) {
        shipments[i as usize] += 1;
    }

    shipments
}

fn main() {
    // Приклад використання функції для підрахунку мінімальної кількості переносу вантажу
    let shipments1 = vec![8, 2, 2, 4, 4];
    let moves1 = count_permutation(&shipments1);
    println!("Мінімальна кількість переносу вантажу: {}", moves1);

    // Приклад, коли неможливо вирівняти вантаж
    let shipments2 = vec![9, 3, 7, 2, 9];
    let moves2 = count_permutation(&shipments2);
    println!("Мінімальна кількість переносу вантажу: {}", moves2);

    // Генерація прикладу
    let n = 5;
    let generated_shipments = gen_shipments(n);
    println!("Згенеровані вантажі: {:?}", generated_shipments);
}
