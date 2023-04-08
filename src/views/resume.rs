use yew::prelude::*;

use crate::components::{
    back_home::BackHome,
    page_header::{PageHeader, PageHeaderProps},
    styled_heading::{StyledHeading},
    timeline::{Timeline, TimelineItemProps},
    transition::Transition, skill::{Skills, SkillProps},
};

#[function_component(Resume)]
pub fn resume() -> Html {
    let page_header_props = use_state(|| PageHeaderProps {
        heading: "Resume".to_owned(),
        hint: "I'm available for hire".to_owned(),
    });

    let education_timeline_items = use_state(|| vec![
        TimelineItemProps {
            start_date: "2019-09-01".to_owned(),
            end_date: Some("2020-06-01".to_owned()),
            title: "Bachelor of Science in Industrial Chemistry".to_owned(),
            more_info: Some("Multimedia University of Kenya".to_owned()),
            description: "This is where I enrolled for my Bachelor's degree in Industrial Chemistry and later on discovered my passion for Computer Science in my second year. It was through a unit in that course entitled \"Introduction to Computer Programming\"".to_owned(),
        },
        // TimelineItemProps {
        //     date: "2015-06-01".to_owned(),
        //     title: "High School".to_owned(),
        //     more_info: Some("Nyang'ori High School".to_owned()),
        //     description: "University of the Philippines Diliman".to_owned(),
        // },
    ]);

    let experience_timeline_items = use_state(|| vec![
        TimelineItemProps {
            start_date: "2020-06-01".to_owned(),
            end_date: None,
            title: "Senior Full-stack Software Engineer - React, NestJS, MySQL, AWS, Vagrant".to_owned(),
            more_info: None,
            description: "Developed web components using React
            Developed endpoints using NestJS framework
            Integrated third-party APIs such as Vungle and AppLovin to deliver ads into mobile games
            Used Vagrant to create and provision EC2 instances in AWS
            Used SQS to queue system jobs to run in the background
            Used S3 bucket to store assets for creating games".to_owned(),
        },
        TimelineItemProps {
            start_date: "2019-06-01".to_owned(),
            end_date: Some("2020-06-01".to_owned()),
            title: "Lead Frontend Engineer - Angular".to_owned(),
            more_info: None,
            description: "Chaired sprint meetings to ensure that tasks were well assigned and completed on time
            Ensured that the goals of the team were achieved fueled by the contribution of each and every member of the team
            Coordinated with other team leads to ensure that the overall mission and vision of the company are met".to_owned(),
        },
        TimelineItemProps {
            start_date: "2018-06-01".to_owned(),
            end_date: Some("2020-06-01".to_owned()),
            title: "Senior Frontend Engineer - Angular".to_owned(),
            more_info: None,
            description: "Worked together with the UI/UX team to ensure that the product designs have been fulfilled to the latter, using Angular and TypeScript, and SCSS for styling the web pages
            Reduced the Angular app build time from 2 hours to ~10 minutes. I achieved this by ensuring reusing of components and reducing redundant code
            Designed and implemented deployment strategy of the frontend application and created pipelines to automate the development/release cycle and containerizing the app using Docker
            Wrote unit tests for components using Jasmine, therefore ensuring that only quality code free from bugs is shipped to production
            Defined the front-end architecture, hence ensuring that the code base is well structured and easy to maintain
            Optimized solutions, and applying suitable algorithms hence ensuring that the application is not expensive to the client in terms of computing resources
            Implemented lazy loading approach for the back office application. The app's performance improved by 50%. I achieved this by breaking the app into smaller lazy-loaded modules
            I orchestrated the containers using Docker Swarm which helped with the high availability of the application".to_owned(),
        },
        TimelineItemProps {
            start_date: "2017-06-01".to_owned(),
            end_date: Some("2020-06-01".to_owned()),
            title: "React Developer (remote)".to_owned(),
            more_info: None,
            description: "Consumed Instagram API to display the company's Instagram activity
            Split the application built in React (Next.js) into smaller components, therefore ensuring that the code base is organized and easier to maintain
            Used the Styled Components library in the project, which ensured that the code was much more concise and declarative".to_owned(),
        },
        TimelineItemProps {
            start_date: "2017-06-01".to_owned(),
            end_date: Some("2020-06-01".to_owned()),
            title: "Junior Web Developer (remote)".to_owned(),
            more_info: None,
            description: "Developed a system to enable students to report bullying cases in the United States of America. This was a mitigation measure for school shootouts as a consequence of bullying, following the Florida School shooting
            Developed a portal to allow students to fill forms for reporting the cases. The cases could be reported anonymously or through an adult on behalf of the student
            Developed a video streaming service that allowed students to upload videos of themselves to the platform
            Developed a mailing service in PHP to enable students to send and receive emails from the platform".to_owned(),
        },
    ]);

    // create state for technical skills
    let technical_skills = use_state(|| vec![
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/typescript.png".to_owned(),
            name: "TypeScript".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/javascript.png".to_owned(),
            name: "JavaScript".to_owned(),
        },
        SkillProps {
            icon: "http://rust-lang.org/logos/rust-logo-128x128.png".to_owned(),
            name: "Rust".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/react-native.png".to_owned(),
            name: "React".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/css3.png".to_owned(),
            name: "CSS".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/nodejs.png".to_owned(),
            name: "NodeJS".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/git.png".to_owned(),
            name: "Git".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/docker.png".to_owned(),
            name: "Docker".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/amazon-web-services.png".to_owned(),
            name: "AWS".to_owned(),
        },
        // angular
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/angularjs.png".to_owned(),
            name: "Angular".to_owned(),
        },
    ]);

    let soft_skills = use_state(|| vec![
        // some soft skills
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/communication.png".to_owned(),
            name: "Communication".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/teamwork.png".to_owned(),
            name: "Teamwork".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/leadership.png".to_owned(),
            name: "Leadership".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/innovation.png".to_owned(),
            name: "Innovation".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/null/create-new.png".to_owned(),
            name: "Creativity".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/decision.png".to_owned(),
            name: "Decision Making".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/000000/organization.png".to_owned(),
            name: "Organization".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/office/16/null/administrator-male--v1.png".to_owned(),
            name: "Management".to_owned(),
        },
        SkillProps {
            icon: "https://img.icons8.com/color/48/null/test-tube.png".to_owned(),
            name: "Analysis".to_owned(),
        },
        
    ]);

    html! {
        <>
            <Transition />
            <main class="resume">
                <BackHome />
                <PageHeader hint={page_header_props.hint.clone()} heading={page_header_props.heading.clone()} />
                <div class="wrapper">
                    <div class="education">
                        <StyledHeading heading={"Education".to_owned()} />
                        <Timeline items={education_timeline_items.to_vec()} />
                    </div>
                    <div class="experience">
                        <StyledHeading heading={"Work Experience".to_owned()} />
                        <Timeline items={experience_timeline_items.to_vec()} />
                    </div>
                </div>

                <div class="skills">
                    <div class="technical">
                        <StyledHeading heading={"Technical Skills".to_owned()} />
                        <Skills skills={technical_skills.to_vec()} />
                    </div>
                    <div class="soft">
                        <StyledHeading heading={"Soft Skills".to_owned()} />
                        <Skills skills={soft_skills.to_vec()} />
                    </div>
                </div>
            </main>
        </>
    }
}
