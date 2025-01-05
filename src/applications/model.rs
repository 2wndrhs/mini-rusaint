use std::collections::HashMap;

// 학기별 성적
#[derive(Debug)]
pub struct SemesterGrade {
    pub year: u32,                 // 학년도
    pub semester: String,          // 학기
    pub attempted_credits: f32,    // 신청학점
    pub earned_credits: f32,       // 취득학점
    pub pf_earned_credits: f32,    // P/F학점
    pub grade_points_average: f32, // 평점평균
    pub grade_points_sum: f32,     // 평점계
    pub arithmetic_mean: f32,      // 산술평균
    pub semester_rank: (u32, u32), // 학기별석차
    pub general_rank: (u32, u32),  // 전체석차
    pub academic_probation: bool,  // 학사경고
    pub consult: bool,             // 상담여부
    pub flunked: bool,             // 유급
}

impl SemesterGrade {
    pub fn from_html_element(tr_element: scraper::ElementRef) -> SemesterGrade {
        let td_selector = scraper::Selector::parse("td").unwrap();
        // 첫 번째 td 요소는 라디오 버튼이므로 skip(1)을 사용하여 제외
        let td_elements: Vec<_> = tr_element.select(&td_selector).skip(1).collect();

        let year_text = td_elements[0].text().collect::<String>();
        let year = year_text.trim().parse().unwrap();

        let semester = td_elements[1].text().collect::<String>().trim().to_string();

        let attempted_credits_text = td_elements[2].text().collect::<String>();
        let attempted_credits = attempted_credits_text.trim().parse().unwrap();

        let earned_credits_text = td_elements[3].text().collect::<String>();
        let earned_credits = earned_credits_text.trim().parse().unwrap();

        let pf_earned_credits_text = td_elements[4].text().collect::<String>();
        let pf_earned_credits = pf_earned_credits_text.trim().parse().unwrap();

        let grade_points_average_text = td_elements[5].text().collect::<String>();
        let grade_points_average = grade_points_average_text.trim().parse().unwrap();

        let grade_points_sum_text = td_elements[6].text().collect::<String>();
        let grade_points_sum = grade_points_sum_text.trim().parse().unwrap();

        let arithmetic_mean_text = td_elements[7].text().collect::<String>();
        let arithmetic_mean = arithmetic_mean_text.trim().parse().unwrap();

        let semester_rank_text = td_elements[8].text().collect::<String>();
        let semester_rank_parts: Vec<&str> = semester_rank_text.trim().split('/').collect();
        let semester_rank = (
            semester_rank_parts[0].parse().unwrap(),
            semester_rank_parts[1].parse().unwrap(),
        );

        let general_rank_text = td_elements[9].text().collect::<String>();
        let general_rank_parts: Vec<&str> = general_rank_text.trim().split('/').collect();
        let general_rank = (
            general_rank_parts[0].parse().unwrap(),
            general_rank_parts[1].parse().unwrap(),
        );

        let academic_probation = td_elements[10].text().collect::<String>().trim() == "Y";
        let consult = td_elements[11].text().collect::<String>().trim() == "Y";
        let flunked = td_elements[12].text().collect::<String>().trim() == "Y";

        SemesterGrade {
            year,
            semester,
            attempted_credits,
            earned_credits,
            pf_earned_credits,
            grade_points_average,
            grade_points_sum,
            arithmetic_mean,
            semester_rank,
            general_rank,
            academic_probation,
            consult,
            flunked,
        }
    }
}

#[derive(Debug)]
pub enum SemesterType {
    FirstSemester,
    SummerSemester,
    SecondSemester,
    WinterSemester,
}

impl SemesterType {
    // sap 내부적으로 사용하는 학기에 대응하는 key
    pub fn key(&self) -> &str {
        match self {
            SemesterType::FirstSemester => "090",
            SemesterType::SummerSemester => "091",
            SemesterType::SecondSemester => "092",
            SemesterType::WinterSemester => "093",
        }
    }
}

// 과목별 성적
#[derive(Debug)]
pub struct CourseGrade {
    pub grade: String,                                // 성적
    pub rating: String,                               // 등급
    pub course_name: String,                          // 과목명
    pub detailed_grade: Option<HashMap<String, f32>>, // 상세성적
    pub course_credits: f32,                          // 과목학점
    pub professor_name: String,                       // 교수명
    pub remarks: String,                              // 비고
    pub course_code: String,                          // 과목코드
}
