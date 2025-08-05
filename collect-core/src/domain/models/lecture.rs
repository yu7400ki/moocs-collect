use crate::domain::models::keys::{CourseKey, LectureKey};

#[derive(Debug, Clone)]
pub struct LectureGroup {
    pub course_key: CourseKey,
    pub name: String,
    pub lectures: Vec<Lecture>,
    pub index: usize,
}

impl LectureGroup {
    pub fn new(
        course_key: CourseKey,
        name: impl Into<String>,
        lectures: Vec<Lecture>,
        index: usize,
    ) -> Self {
        Self {
            course_key,
            name: name.into(),
            lectures,
            index,
        }
    }

    pub fn builder() -> LectureGroupBuilder {
        LectureGroupBuilder::new()
    }

    pub fn display_name(&self) -> &str {
        &self.name
    }
}

/// Lecture group builder
#[derive(Debug, Clone)]
pub struct LectureGroupBuilder {
    course_key: Option<CourseKey>,
    name: Option<String>,
    lectures: Option<Vec<Lecture>>,
    index: Option<usize>,
}

impl LectureGroupBuilder {
    pub fn new() -> Self {
        Self {
            course_key: None,
            name: None,
            lectures: None,
            index: None,
        }
    }

    pub fn with_course_key(mut self, course_key: CourseKey) -> Self {
        self.course_key = Some(course_key);
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_lectures(mut self, lectures: Vec<Lecture>) -> Self {
        self.lectures = Some(lectures);
        self
    }

    pub fn with_index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn build(self) -> Option<LectureGroup> {
        Some(LectureGroup::new(
            self.course_key?,
            self.name?,
            self.lectures?,
            self.index?,
        ))
    }
}

/// Lecture domain model
#[derive(Debug, Clone)]
pub struct Lecture {
    pub key: LectureKey,
    pub name: String,
    pub index: usize,
}

impl Lecture {
    pub fn new(key: LectureKey, name: impl Into<String>, index: usize) -> Self {
        Self {
            key,
            name: name.into(),
            index,
        }
    }

    pub fn builder() -> LectureBuilder {
        LectureBuilder::new()
    }

    /// Get display name, falling back to slug if name is empty
    pub fn display_name(&self) -> &str {
        if self.name.is_empty() {
            self.key.slug.value()
        } else {
            &self.name
        }
    }
}

/// Lecture builder
#[derive(Debug, Clone)]
pub struct LectureBuilder {
    key: Option<LectureKey>,
    name: Option<String>,
    index: Option<usize>,
}

impl LectureBuilder {
    pub fn new() -> Self {
        Self {
            key: None,
            name: None,
            index: None,
        }
    }

    pub fn with_key(mut self, key: LectureKey) -> Self {
        self.key = Some(key);
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn build(self) -> Option<Lecture> {
        Some(Lecture::new(self.key?, self.name?, self.index?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::keys::{CourseKey, CourseSlug, LectureSlug, Year};

    #[test]
    fn test_lecture_builder() {
        let year = Year::new(2023).unwrap();
        let course_slug = CourseSlug::new("course").unwrap();
        let lecture_slug = LectureSlug::new("lecture").unwrap();

        let course_key = CourseKey::new(year, course_slug);
        let lecture_key = LectureKey::new(course_key, lecture_slug);

        let lecture = LectureBuilder::new()
            .with_key(lecture_key.clone())
            .with_name("Test Lecture")
            .with_index(0)
            .build()
            .unwrap();

        assert_eq!(lecture.key, lecture_key);
        assert_eq!(lecture.name, "Test Lecture");
        assert_eq!(lecture.index, 0);
    }

    #[test]
    fn test_lecture_group_builder() {
        let year = Year::new(2023).unwrap();
        let course_slug = CourseSlug::new("course").unwrap();
        let lecture_slug1 = LectureSlug::new("lecture1").unwrap();
        let lecture_slug2 = LectureSlug::new("lecture2").unwrap();

        let course_key = CourseKey::new(year, course_slug);
        let lecture_key1 = LectureKey::new(course_key.clone(), lecture_slug1);
        let lecture_key2 = LectureKey::new(course_key.clone(), lecture_slug2);

        let lecture1 = LectureBuilder::new()
            .with_key(lecture_key1)
            .with_name("Lecture 1")
            .with_index(0)
            .build()
            .unwrap();

        let lecture2 = LectureBuilder::new()
            .with_key(lecture_key2)
            .with_name("Lecture 2")
            .with_index(1)
            .build()
            .unwrap();

        let lectures = vec![lecture1, lecture2];

        let lecture_group = LectureGroupBuilder::new()
            .with_course_key(course_key.clone())
            .with_name("Group A")
            .with_lectures(lectures.clone())
            .with_index(0)
            .build()
            .unwrap();

        assert_eq!(lecture_group.course_key, course_key);
        assert_eq!(lecture_group.name, "Group A");
        assert_eq!(lecture_group.lectures.len(), 2);
        assert_eq!(lecture_group.lectures[0].name, "Lecture 1");
        assert_eq!(lecture_group.lectures[0].index, 0);
        assert_eq!(lecture_group.lectures[1].name, "Lecture 2");
        assert_eq!(lecture_group.lectures[1].index, 1);
        assert_eq!(lecture_group.index, 0);
    }
}
