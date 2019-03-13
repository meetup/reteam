use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use structopt::StructOpt;

const TAGGED_REPOS: &str = r#"
query TaggedRepos($query: String!) {
  search(query: $query, type: REPOSITORY, first: 100) {
    repositoryCount
    nodes {
      ... on Repository {
        name,
        id
      }
    }
  }
}
"#;

const TAG_REPO: &str = r#"
mutation TagRepo($repoId: ID!, $topicName: String!) {
  updateTopics(input: {repositoryId: $repoId, topicNames: [$topicName]}) {
    repository {
      name,
      id
    }
  }
}
"#;

#[derive(Deserialize)]
struct Env {
    github_token: String,
}

#[derive(StructOpt)]
#[structopt(
    name = "reteam",
    about = "tool for managing updates to team owned github repositories"
)]
enum Options {
    #[structopt(name = "repos", about = "List repos with a team tag")]
    Repos {
        #[structopt(short = "o", long = "organization")]
        organization: String,
        #[structopt(short = "t", long = "team")]
        team: String,
    },
    #[structopt(name = "update-topic", about = "Update github repository team topic")]
    UpdateTopic {
        #[structopt(short = "o", long = "organization")]
        organization: String,
        #[structopt(short = "t", long = "team")]
        team: String,
        #[structopt(short = "n", long = "new-team")]
        topic: String,
    },
}

#[derive(Deserialize, Debug)]
struct Response<D> {
    data: D,
}

#[derive(Deserialize, Debug)]
struct Nodes<N> {
    nodes: Vec<N>,
}

#[derive(Deserialize, Debug)]
struct Search {
    search: Nodes<Repo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UpdateTopics {
    update_topics: RepositoryNode,
}

#[derive(Deserialize, Debug)]
struct RepositoryNode {
    repository: Repo,
}

#[derive(Deserialize, Debug)]
struct Repo {
    id: String,
    name: String,
}

fn request<V, O>(
    github_token: &str,
    query: &str,
    variables: V,
) -> Result<Response<O>, Box<dyn Error>>
where
    V: Serialize,
    O: DeserializeOwned,
{
    Ok(Client::new()
        .post("https://api.github.com/graphql")
        .header("Authorization", format!("Bearer {}", github_token))
        .json(&json!(
            {
                "query": query,
                "variables": variables
            }
        ))
        .send()?
        .json::<Response<O>>()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let Env { github_token } = envy::from_env::<Env>()?;
    match Options::from_args() {
        Options::Repos { organization, team } => {
            let results: Response<Search> = request(
                github_token.as_str(),
                TAGGED_REPOS,
                &json!({ "query": format!("org:{} topic:{}", organization, team) }),
            )?;
            for node in results.data.search.nodes {
                println!("{}", node.name);
            }
        }
        Options::UpdateTopic {
            organization,
            team,
            topic,
        } => {
            let results: Response<Search> = request(
                github_token.as_str(),
                TAGGED_REPOS,
                json!({ "query": format!("org:{} topic:{}", organization, team) }),
            )?;
            for node in results.data.search.nodes {
                let result: Response<UpdateTopics> = request(
                    github_token.as_str(),
                    TAG_REPO,
                    json!(
                        {
                            "repoId": node.id,
                            "topicName": topic
                        }
                    ),
                )?;
                println!(
                    "updated {} topic from {} to {}",
                    result.data.update_topics.repository.name, team, topic
                );
            }
        }
    }
    Ok(())
}
