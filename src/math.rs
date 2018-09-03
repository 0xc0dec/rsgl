use glm;

// TODO better
pub fn equal(a: f32, b: f32) -> bool {
    glm::all(&glm::equal(&glm::vec1(a), &glm::vec1(b)))
}
