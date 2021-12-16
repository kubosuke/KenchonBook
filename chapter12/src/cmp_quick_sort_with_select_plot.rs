use plotters::prelude::*;
use rand::Rng;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ns = vec![10, 100, 1000, 10000, 50000, 100000, 500000, 1000000, 2000000, 4000000];

    let mut rng = rand::thread_rng();

    let mut quick_sort_time = vec![];
    let mut select_time = vec![];

    for n in ns {
        let k = rng.gen_range(0..n);
        let mut v1: Vec<i32> = (0..n).map(|_| rng.gen_range(0..=4000000)).collect();
        let v2 = v1.clone();

        // 1. quick
        let start = Instant::now();
        quick_sort(&mut v1, 0, n);
        let quick_sort_kth = v1[k];
        let duration = start.elapsed();
        quick_sort_time.push(((n as f32).log10(), duration.as_secs_f32()));

        // 2. select
        let start = Instant::now();
        let select_kth = select_kth(v2, k);
        let duration = start.elapsed();
        select_time.push(((n as f32).log10(), duration.as_secs_f32()));

        assert_eq!(quick_sort_kth, select_kth);
    }

    let root = BitMapBackend::new("plotters-doc-data/result.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&root)
        .caption("sec / log_10(number of elements)", ("sans-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..10f32, 0f32..10f32)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // plot quick_sort()
    chart.draw_series(LineSeries::new(
        quick_sort_time.clone(),
        &RED,
    ))?;
    chart.draw_series(PointSeries::of_element(
        quick_sort_time,
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
                + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;

    // plot select()
    chart.draw_series(LineSeries::new(
        select_time.clone(),
        &BLUE,
    ))?;
    chart.draw_series(PointSeries::of_element(
        select_time,
        5,
        &BLUE,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
                + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;
    Ok(())
}


fn quick_sort(v: &mut Vec<i32>, left: usize, right: usize) {
    // do procedure more than 3 elements exist
    if right - left < 2 {
        return
    }

    // put pivot in the middle
    let pivot_index = (left + right) / 2;
    let pivot = v[pivot_index];

    // swap pivot to the right most side
    v.swap(pivot_index, right-1);

    // seek left -> right
    let mut i = left;
    for j in left..right - 1 {
        if v[j] < pivot {
            // move smaller one to the left side
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, right-1);

    // after that, each elements placed like below:
    // [smaller than pivot(1)][pivot][larger than pivot(2)]
    // do procedure in (1) and (2)
    quick_sort(v, left, i);
    quick_sort(v, i+1, right);
}


// selects the kth smallest element
fn select_kth(mut a: Vec<i32>, k: usize) -> i32 {
    // 1. if length of a is less than 100, return kth smallest value
    if a.len() <= 100 {
        a.sort();
        if a.len() == k {
            return a[k-1]
        }
        return a[k]
    };

    // 2. find a good pivot
    // split every 5 elements and find mid of each chunks
    //
    //  mid1                           mid2
    //  |                               |
    //  V                               V
    // |8, 5, 12, 81, 3|18, 36, 12, 7, 14|...
    //
    let mut a2: Vec<i32> = vec![];
    for i in (0..a.len()).into_iter().step_by(5) {
        let mut a3 = if i + 5 > a.len() {
            a[i..].to_vec()
        } else {
            a[i..i+5].to_vec()
        };
        a3.sort();
        a2.push(a3[a3.len()/2]);
    };


    // select mid of a2
    // second args is a.len()/10: 'cause a.len() = a2.len() * 5
    let mid = select_kth(a2, a.len()/10);

    // after you find mid, let's divide among 3 parts:
    // p: less than mid
    // q: equals mid
    // r: more than mid
    let mut p: Vec<i32> = vec![];
    let mut q: Vec<i32> = vec![];
    let mut r: Vec<i32> = vec![];

    for i in 0..a.len() {
        if a[i] < mid {
            p.push(a[i]);
        } else if a[i] == mid {
            q.push(a[i]);
        } else {
            r.push(a[i]);
        }
    };

    if k <= p.len() {
        return select_kth(p, k);
    } else if k <= p.len() + q.len() {
        return mid;
    } else {
        return select_kth(r, k - p.len() - q.len())
    };
}
