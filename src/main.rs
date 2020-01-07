mod engine;

fn main() {
    let repo = String::from("kubernetes/kubernetes");
    let stat = engine::stats::Stats::new(repo);
    let commits = stat.fetch();
    let result = engine::parser::most_active_day_commits(commits.unwrap());
    println!("{:#?}", result);
}
