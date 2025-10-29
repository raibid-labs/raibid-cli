you are a distinguised engineer at nvidia specializing in developer tooling. You have been tasked with creating a suite of tooling to assist invidual developers in getting the most out of their nvidia dgx spark.
For this specific tool, "raibid-ci", the goal is to simplify and streamline the process of provisioning, managing, and tearing down a self-hosted pool of CI agents running on a NVIDIA DGX Spark. Use the following context to guide your planning.

# summary
Initialize and manage a self-hosted pool of ci agents running on an nvidia dgx spark.

## Characteristics
- DX-fist
- TUI-native
- ephemeral
- auto-scaling
- plugin based architecture

## Required Technologies
- keda
- k3s
- gitea
- flux cli
- ratatui
- nushell
- redis streams (for job queue but open to alternatives)
- rust (for api and tui client)
- research and determine what else is needed (e.g. dhall/cue/tanka for cluster config management? Is there a nushell and/or rust k8s client library we can use?)

# requirements
- research, notes, diagrams, etc should all be stored under ./docs
- markdown documentation should include links to relevant resources and references both internal and external
- create and use ./docs/work/ directory for organizing milstones, issues, and tasks. Use markdown files for this purpose and assume that the files will eventually be submitted to github using their content as issue descriptions
- create mermaid diagrams where appropriate and store them under ./docs/diagrams/
- try to use terse language, bullet points, mermaid diagrams, and links/references wherever possible to keep things concise and scannable

# instructions

1. anaylze the old_readme.md file to get an idea for what we are building. Ultrathink, research, and fill in any gaps in knowledge you have about the required technologies. Create and update the repo's README.md file as needed to ensure it accurately reflects the project goals, architecture, and specifications.

2. create a detailed project plan in ./docs/work/plan.md that breaks down the project into milestones, issues, and tasks. Each milestone should have a clear goal and each issue should be broken down into manageable tasks. Use markdown formatting to organize the content clearly. The plan should include a table of contents and outline for the issues and tasks.

3. stop once the project plan is complete. Do not proceed to implementation or coding at this time.

4. initialize a CLAUDE.md file in the root directory with appropriate content for an open source project hosted on github, then commit and push all changes up this point as a single commit with the message "chore: initial project plan and documentation"