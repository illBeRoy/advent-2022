use crate::day::Day;
use crate::input::input_for_day;

#[derive(Clone, Copy)]
pub struct Day8 {}

impl Day for Day8 {
    fn title(&self) -> &'static str {
        "Treetop Tree House"
    }

    fn description(&self) -> &'static str {
        "
        First, we parse. We turn the grid into a matrix of Trees.

        For the first task, we need to identify the trees visible from outside. In order to achieve that, for each tree,
        we identify the highest tree it can see from every side.
        
        Let's say we have N trees in a line. We start from the first tree - there is no highest tree next to it so its value is -1.
        The next tree only has one tree in front of it, so that tree is the highest it sees.
        The third tree looks at the second one: if the second one is shorter than the first one, it takes the first one as the highest.
        Otherwise, it takes the second.

        Generally speaking, the highest tree that comes before any n<N tree, is either the highest tree that comes before the n-1 tree,
        or the n-1 tree itself - the highest of the two.

        In order to translate that into what we need, we actually run over the entire matrix twice more:
        The first time we go from the top left, and use the above method to calculate each tree's highest tree from top and left.
        The second time we go from the bottom right, and do the same for the bottom and right values.

        Finally, we iterate over the entire matrix and identify which trees are actually higher than all higher trees from all directions.
        These are the visible trees, and the count of those is the answer for task 1.

        As for task 2, there is an efficient O(n) solution, where we keep the distance from every possible height for each tree.
        Since the height is 0-9, the complexity for storing all possible height is O(1).

        I did not do that. Instead, I opted to do the naive O(n*sqrt(n)) solution where we just run over the entire forest and calculate
        each tree's score individually. Tough luck.
        "
    }

    fn task_1(&self) -> String {
        let input = input_for_day(8);
        let matrix = parse_input_into_forest(&input);

        let visible_trees = matrix
            .iter()
            .flatten()
            .filter(|tree| tree.is_visible_from_outside());

        format!(
            "count of trees visible from the outside is {}",
            visible_trees.count()
        )
    }

    fn task_2(&self) -> String {
        let input = input_for_day(8);
        let matrix = parse_input_into_forest(&input);

        let all_tress = matrix.iter().flatten();

        let highest_score = all_tress
            .map(|tree| tree.get_score(&matrix))
            .max()
            .expect("for some reason, no tree was hidden?");

        format!("the highest score for a hidden tree is {}", highest_score,)
    }
}

type Forest = Vec<Vec<Tree>>;

struct Tree {
    position: (usize, usize),
    height: i8,
    highest_from_left: i8,
    highest_from_top: i8,
    highest_from_right: i8,
    highest_from_bottom: i8,
}

impl Tree {
    fn new(position: (usize, usize), height: i8) -> Self {
        Self {
            position,
            height,
            highest_from_left: -1,
            highest_from_top: -1,
            highest_from_right: -1,
            highest_from_bottom: -1,
        }
    }

    fn is_visible_from_outside(&self) -> bool {
        self.height > self.highest_from_left
            || self.height > self.highest_from_top
            || self.height > self.highest_from_right
            || self.height > self.highest_from_bottom
    }

    fn get_score(&self, matrix: &Forest) -> usize {
        let width = matrix[0].len();
        let height = matrix.len();
        let (x, y) = self.position;

        let left_distance = x
            - (0..x)
                .rev()
                .find(|other_x| matrix[y][*other_x].height >= self.height)
                .unwrap_or(0);
        let right_distance = ((x + 1)..width)
            .find(|other_x| matrix[y][*other_x].height >= self.height)
            .unwrap_or(width - 1)
            - x;
        let top_distance = y
            - (0..y)
                .rev()
                .find(|other_y| matrix[*other_y][x].height >= self.height)
                .unwrap_or(0);
        let bottom_distance = ((y + 1)..height)
            .find(|other_y| matrix[*other_y][x].height >= self.height)
            .unwrap_or(height - 1)
            - y;

        left_distance * top_distance * right_distance * bottom_distance
    }
}

fn parse_input_into_forest(input: &String) -> Forest {
    let mut matrix: Forest = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<Tree> = vec![];

        for (x, char) in line.chars().enumerate() {
            let height = char
                .to_string()
                .parse::<i8>()
                .expect(format!("char is not a valid digit: {}", char).as_str());

            row.push(Tree::new((x, y), height));
        }

        matrix.push(row);
    }

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if y > 0 {
                let tree_from_top = &matrix[y - 1][x];

                matrix
                    .get_mut(y)
                    .and_then(|row| row.get_mut(x))
                    .unwrap()
                    .highest_from_top = tree_from_top.highest_from_top.max(tree_from_top.height);
            }

            if x > 0 {
                let tree_from_left = &matrix[y][x - 1];

                matrix
                    .get_mut(y)
                    .and_then(|row| row.get_mut(x))
                    .unwrap()
                    .highest_from_left =
                    tree_from_left.highest_from_left.max(tree_from_left.height);
            }
        }
    }

    for y in (0..matrix.len()).rev() {
        for x in (0..matrix[0].len()).rev() {
            if y < matrix.len() - 1 {
                let tree_from_bottom = &matrix[y + 1][x];

                matrix
                    .get_mut(y)
                    .and_then(|row| row.get_mut(x))
                    .unwrap()
                    .highest_from_bottom = tree_from_bottom
                    .highest_from_bottom
                    .max(tree_from_bottom.height);
            }

            if x < matrix[0].len() - 1 {
                let tree_from_right = &matrix[y][x + 1];

                matrix
                    .get_mut(y)
                    .and_then(|row| row.get_mut(x))
                    .unwrap()
                    .highest_from_right = tree_from_right
                    .highest_from_right
                    .max(tree_from_right.height);
            }
        }
    }

    matrix
}
