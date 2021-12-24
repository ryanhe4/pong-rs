pub struct SceneBase {
  components: Vec<u32>,
}

trait Draw {
  fn draw();
  fn add_component();
}