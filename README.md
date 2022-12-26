# LowEndInsight CLI

## Status
VERY MUCH JUST ALPHA!!

Basic async POST and GET tested on

* Linux (Ubuntu AMD64)
* Windows 11

## Running

* Download the executable for the latest [release](https://github.com/quency-ai/lowendinsight-cli/releases) (for your target arch)
* Create account at [RapidAPI.com](https://rapidapi.com)
* Subscribe to `lowendinsight` @ [https://rapidapi.com/quency-ai-quency-ai-default/api/lowendinsight2](https://rapidapi.com/quency-ai-quency-ai-default/api/lowendinsight2)
* Grab your token
* Paste into your/a $HOME/.config/lei/config.toml
  * Should look like: 
``` toml
  rapid_key = "INSERT YOUR KEY HERE"
```
* Run a command! `lei help`

``` sh
lowendinsight-cli on  develop [!] is 📦 v0.0.4 via 🦀 v1.66.0 
❯ lei --help
lei 0.0.4
A command-line interface to LowEndInsight (https://lowendinsight.dev)

USAGE:
    lei [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config-file <config-file>    Use a different configuration, other than default, file

SUBCOMMANDS:
    analyze         Run LowEndInsight against a git repository
    get-analysis    Retreive LowEndInsight analysis with a UUID
    help            Prints this message or the help of the given subcommand(s)

lowendinsight-cli on  develop [!] is 📦 v0.0.4 via 🦀 v1.66.0 
❯ lei analyze -v 0 https://github.com/bitwarden/server
{"metadata":{"times":{"duration":0,"end_time":"","start_time":"2022-12-26T15:34:17.395182Z"}},"report":{"repos":[{"data":{"repo":"https://github.com/bitwarden/server"}}]},"state":"incomplete","uuid":"c25afdb6-8532-11ed-a137-be6b9608d8bb"}

lowendinsight-cli on  develop [!] is 📦 v0.0.4 via 🦀 v1.66.0 
❯ lei get-analysis -v 0 c25afdb6-8532-11ed-a137-be6b9608d8bb | jq
{
  "metadata": {
    "times": {
      "duration": 0,
      "end_time": "2022-12-26T15:34:38.910473Z",
      "start_time": "2022-12-26T15:34:17.395182Z"
    }
  },
  "report": {
    "repos": [
      {
        "data": {
          "config": {
            "base_temp_dir": "/tmp",
            "critical_contributor_level": 2,
            "critical_currency_level": 104,
            "critical_functional_contributors_level": 2,
            "critical_large_commit_level": 0.4,
            "high_contributor_level": 3,
            "high_currency_level": 52,
            "high_functional_contributors_level": 3,
            "high_large_commit_level": 0.3,
            "medium_contributor_level": 5,
            "medium_currency_level": 26,
            "medium_functional_contributors_level": 5,
            "medium_large_commit_level": 0.2,
            "sbom_risk_level": "medium"
          },
          "git": {
            "default_branch": "refs/remotes/origin/master",
            "hash": "c39fb8f7af58ed3c99111d429ec822e338f98bb2"
          },
          "project_types": {
            "node": [
              "/tmp/lei-1672068857-85-1654kc0/server/bitwarden_license/src/Commercial.Core/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/bitwarden_license/src/Scim/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/bitwarden_license/src/Sso/package-lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/bitwarden_license/src/Sso/package.json",
              "/tmp/lei-1672068857-85-1654kc0/server/bitwarden_license/src/Sso/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/bitwarden_license/test/Commercial.Core.Test/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/bitwarden_license/test/Scim.IntegrationTest/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/bitwarden_license/test/Scim.Test/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/perf/MicroBenchmarks/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/Admin/package-lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/Admin/package.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/Admin/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/Api/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/Billing/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/Core/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/Events/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/EventsProcessor/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/Icons/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/Identity/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/Infrastructure.Dapper/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/Infrastructure.EntityFramework/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/Notifications/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/src/SharedWeb/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/test/Api.IntegrationTest/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/test/Api.Test/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/test/Billing.Test/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/test/Common/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/test/Core.Test/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/test/Icons.Test/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/test/Identity.IntegrationTest/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/test/Identity.Test/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/test/Infrastructure.EFIntegration.Test/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/test/Infrastructure.IntegrationTest/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/test/IntegrationTestCommon/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/util/Migrator/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/util/MySqlMigrations/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/util/PostgresMigrations/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/util/Server/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/util/Setup/packages.lock.json",
              "/tmp/lei-1672068857-85-1654kc0/server/util/SqliteMigrations/packages.lock.json"
            ]
          },
          "repo": "https://github.com/bitwarden/server",
          "repo_size": "0",
          "results": {
            "commit_currency_risk": "low",
            "commit_currency_weeks": 0,
            "contributor_count": 127,
            "contributor_risk": "low",
            "functional_contributor_names": [
              "Oscar Hinton <oscar@oscarhinton.com>",
              "Chad Scharf <3904944+cscharf@users.noreply.github.com>",
              "Addison Beck <abeck@bitwarden.com>",
              "Matt Gibson <mgibson@bitwarden.com>",
              "Mart124 <37041094+Mart124@users.noreply.github.com>",
              "Joseph Flinn <58369717+joseph-flinn@users.noreply.github.com>",
              "Vince Grassia <593223+vgrassia@users.noreply.github.com>",
              "Kyle Spearrin <kspearrin@users.noreply.github.com>",
              "Kyle Spearrin <kyle.spearrin@gmail.com>",
              "Thomas Rittson <31796059+eliykat@users.noreply.github.com>",
              "Vincent Salucci <26154748+vincentsalucci@users.noreply.github.com>",
              "github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>"
            ],
            "functional_contributors": 12,
            "functional_contributors_risk": "low",
            "large_recent_commit_risk": "low",
            "recent_commit_size_in_percent_of_codebase": 0.00998,
            "sbom_risk": "medium",
            "top10_contributors": [
              {
                "contributions": 2360,
                "email": "kyle.spearrin@gmail.com",
                "merges": 21,
                "name": "Kyle Spearrin"
              },
              {
                "contributions": 153,
                "email": "kspearrin@users.noreply.github.com",
                "merges": 2,
                "name": "Kyle Spearrin"
              },
              {
                "contributions": 148,
                "email": "3904944+cscharf@users.noreply.github.com",
                "merges": 20,
                "name": "Chad Scharf"
              },
              {
                "contributions": 100,
                "email": "mgibson@bitwarden.com",
                "merges": 0,
                "name": "Matt Gibson"
              },
              {
                "contributions": 95,
                "email": "oscar@oscarhinton.com",
                "merges": 0,
                "name": "Oscar Hinton"
              },
              {
                "contributions": 90,
                "email": "31796059+eliykat@users.noreply.github.com",
                "merges": 0,
                "name": "Thomas Rittson"
              },
              {
                "contributions": 77,
                "email": "58369717+joseph-flinn@users.noreply.github.com",
                "merges": 0,
                "name": "Joseph Flinn"
              },
              {
                "contributions": 61,
                "email": "593223+vgrassia@users.noreply.github.com",
                "merges": 0,
                "name": "Vince Grassia"
              },
              {
                "contributions": 55,
                "email": "abeck@bitwarden.com",
                "merges": 0,
                "name": "Addison Beck"
              },
              {
                "contributions": 48,
                "email": "26154748+vincentsalucci@users.noreply.github.com",
                "merges": 0,
                "name": "Vincent Salucci"
              }
            ]
          },
          "risk": "medium"
        },
        "header": {
          "duration": 7,
          "end_time": "2022-12-26T15:34:24.791899Z",
          "library_version": "0.7.3",
          "repo": "https://github.com/bitwarden/server",
          "source_client": "lei_worker",
          "start_time": "2022-12-26T15:34:17.446776Z",
          "uuid": "c6c3a4e8-8532-11ed-b22c-0a70d9954673"
        }
      }
    ]
  },
  "state": "complete",
  "uuid": "c25afdb6-8532-11ed-a137-be6b9608d8bb"
}
```
  
## TODOS

- Implement! :D
  - Submit by URL
    - Sync
  - Submit by URLs
    - Async
    - Sync
- Test all the capes! :D
- Document more!  :D

## Tips

**Pipe to `jq`?  Set `-v 0`:**

`lei analyze -v 0 https://github.com/kitplummer/gbtestee | jq`
