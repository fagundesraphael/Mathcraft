pub struct AverageProblem {
    pub number_of_people: u32,
    pub group_average: f64,
}

pub struct AverageProblemWithIncognito {
    pub initial_group_average: AverageProblem,
    pub new_total_average: f64,
}

impl AverageProblem {
    pub fn new(count: u32, average: f64) -> Self {
        Self {
            number_of_people: count,
            group_average: average,
        }
    }

    pub fn calculate_total(&self) -> f64 {
        self.group_average * self.number_of_people as f64
    }
}

impl AverageProblemWithIncognito {
    pub fn new(initial_people: u32, initial_average: f64, new_average: f64) -> Self {
        Self {
            initial_group_average: AverageProblem::new(initial_people, initial_average),
            new_total_average: new_average,
        }
    }

    pub fn calculate_new_average(&self) -> f64 {
        let new_total =
            self.new_total_average * (self.initial_group_average.number_of_people + 1) as f64;
        let initial_total = self.initial_group_average.calculate_total();

        new_total - initial_total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_calculation() {
        let problem = AverageProblem::new(3, 90.0);
        assert_eq!(problem.calculate_total(), 270.0);
    }

    #[test]
    fn test_basic_average() {
        let problem = AverageProblem::new(2, 85.0);
        assert_eq!(problem.calculate_total(), 170.0);
    }

    #[test]
    fn test_new_person_average() {
        let problem = AverageProblemWithIncognito::new(2, 85.0, 90.0);
        assert_eq!(problem.calculate_new_average(), 100.0);
    }

    #[test]
    fn test_another_scenario() {
        let problem = AverageProblemWithIncognito::new(3, 70.0, 75.0);
        assert_eq!(problem.calculate_new_average(), 90.0);
    }
}
