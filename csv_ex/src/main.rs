const MOCK_DATA: &'static str = include_str!("csv_test.csv");

struct Groups<'a> {
    inner: Vec<&'a str>,
}

struct Projects<'a> {
    inner: Vec<&'a str>,
}

struct AppIDs<'a> {
    inner: Vec<&'a str>,
}

#[derive(Debug)]
struct AppInfo {
    group: String,
    project: String,
    appid: String,
}

impl AppInfo {
    fn generate_vec(groups: Groups, projects: Projects, appids: AppIDs) -> Vec<AppInfo> {
        let items = groups
            .inner
            .iter()
            .zip(projects.inner.iter())
            .zip(appids.inner.iter());

        let mut res_vec = Vec::new();
        for item in items.filter(|item| item.0.1.to_string().eq("ai.pegasus")) {
            res_vec.push(AppInfo {
                group: item.0.0.to_string(),
                project: item.0.1.to_string(),
                appid: item.1.to_string().trim().to_string(),
            });
        }

        res_vec
    }
}

fn main() {
    let data: Vec<_> = MOCK_DATA.split("\n").skip(1).collect();
    // println!("{:?}", data);

    let groups: Vec<_> = data
        .iter()
        .flat_map(|line| line.split(",").nth(1))
        .collect();
    //  println!("{:?}", groups);
    let groups = Groups { inner: groups };

    let projects: Vec<_> = data
        .iter()
        .flat_map(|line| line.split(",").nth(2))
        .collect();
    // println!("{:?}", projects);
    let projects = Projects { inner: projects };

    let appids: Vec<_> = data
        .iter()
        .flat_map(|line| line.split(",").nth(3))
        .collect();
    // println!("{:?}", appids);
    let appids = AppIDs { inner: appids };

    let infos = AppInfo::generate_vec(groups, projects, appids);
    println!("{:?}", infos)
}
