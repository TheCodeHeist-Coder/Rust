pub mod ifelse;
pub mod testwhile;
pub mod testfor;

fn main() {
    println!("Control Flows!");

    ifelse::if_else_concepts::test_if();
    testwhile::testing_while_loop::test_while();
    testfor::testing_for_loop::test_for_loop();

}



