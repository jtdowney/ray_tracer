use ray_tracer::{Matrix4, identity_matrix, matrix, point};

fn main() {
    println!("Chapter 3: Matrix Explorations\n");

    // 1. What happens when you invert the identity matrix?
    println!("1. Inverting the identity matrix:");
    let identity: Matrix4 = identity_matrix();
    let inverted_identity = identity.inverse().unwrap();
    println!("   Identity matrix:");
    print_matrix(&identity);
    println!("   Inverse of identity:");
    print_matrix(&inverted_identity);
    println!("   Result: The inverse of the identity matrix is the identity matrix.\n");

    // 2. What do you get when you multiply a matrix by its inverse?
    println!("2. Multiplying a matrix by its inverse:");
    let a = matrix([[3, -9, 7, 3], [3, -8, 2, -9], [-4, 4, 4, 1], [-6, 5, -1, 1]]);
    let a_inverse = a.inverse().unwrap();
    let product = a * a_inverse;
    println!("   Matrix A:");
    print_matrix(&a);
    println!("   A * A^(-1):");
    print_matrix(&product);
    println!("   Result: You get the identity matrix.\n");

    // 3. Is there any difference between the inverse of the transpose of a matrix,
    //    and the transpose of the inverse?
    println!("3. Inverse of transpose vs transpose of inverse:");
    let b = matrix([[6, 4, 4, 4], [5, 5, 7, 6], [4, -9, 3, -7], [9, 1, 7, -6]]);
    let inverse_of_transpose = b.transpose().inverse().unwrap();
    let transpose_of_inverse = b.inverse().unwrap().transpose();
    println!("   Matrix B:");
    print_matrix(&b);
    println!("   Inverse of transpose:");
    print_matrix(&inverse_of_transpose);
    println!("   Transpose of inverse:");
    print_matrix(&transpose_of_inverse);
    println!("   Result: They are the same.\n");

    // 4. Multiplying a modified identity matrix by a tuple
    println!("4. Modifying identity matrix and multiplying by a tuple:");
    let p = point(1, 2, 3);
    println!("   Original point: ({}, {}, {})", p.x(), p.y(), p.z());
    println!(
        "   Identity * point: ({}, {}, {})",
        (identity * p).x(),
        (identity * p).y(),
        (identity * p).z()
    );

    let mut modified = identity;
    modified[(1, 1)] = 2.0;
    println!("\n   Modified identity (element [1,1] changed to 2):");
    print_matrix(&modified);
    let result = modified * p;
    println!(
        "   Modified * point: ({}, {}, {})",
        result.x(),
        result.y(),
        result.z()
    );
    println!("   Result: The y component is doubled (scaled by 2).");
}

fn print_matrix(m: &Matrix4) {
    for row in 0..4 {
        print!("   | ");
        for col in 0..4 {
            print!("{:8.4} ", m[(row, col)]);
        }
        println!("|");
    }
}
