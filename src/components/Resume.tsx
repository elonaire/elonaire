import { FunctionComponent } from "react";

interface ResumeProps {
    
}
 
const Resume: FunctionComponent<ResumeProps> = () => {
    return (
        <>
        <section id="section-resume" className="dark">
                    <div className="container">
                        <div className="row align-items-center">
                            <div className="col-lg-12 wow fadeInRight">
                                <h4 className="title">I'm available for hire</h4>
                                <h2 className="title">My Resume</h2>
                                <div className="title-underline"></div>
                            </div>
                        </div>
                        <div className="row">
                            <div className="col-md-6 wow fadeInRight" data-wow-delay=".25s">
                                <h3 className="s_border">Education</h3>
                                <ul className="d_timeline">
                                    <li className="d_timeline-item">
                                        <h3 className="d_timeline-title">Sep 2014 - Dec 2018</h3>
                                        <p className="d_timeline-text"><span className="d_title">
                                            Bachelor of Science - BS, Industrial Chemistry<span>Multimedia University of Kenya</span></span>This is where I enrolled for my Bachelor's degree in Industrial Chemistry and later on discovered my passion for Computer Science in my second year. It was through a unit in that course titled "Introduction to Computer Programming".</p>
                                    </li>
                                </ul>
                            </div>
                            <div className="col-md-6 wow fadeInRight" data-wow-delay=".5s">
                                <h3 className="s_border">Work Experiences</h3>
                                <ul className="d_timeline">
                                    <li className="d_timeline-item">
                                        <h3 className="d_timeline-title">Dec 2021 - present</h3>
                                        <p className="d_timeline-text"><span className="d_title">Senior Full-stack Software Engineer - React, NestJS, MySQL, AWS, Vagrant<span>Turing</span></span>
                                        <ul>
                                            <li>Developed web components using React.</li>
                                            <li>Developed endpoints using NestJS framework.</li>
                                            <li>Integrated third-party APIs such as Vungle and AppLovin to deliver ads into mobile games.</li>
                                            <li>Used Vagrant to create and provision EC2 instances in AWS</li>
                                            <li>Used SQS to queue system jobs to run in the background</li>
                                            <li>Used S3 bucket to store assets for creating games</li>
                                        </ul></p>
                                    </li>
                                    <li className="d_timeline-item">
                                        <h3 className="d_timeline-title">Aug 2021 - Jan 2022</h3>
                                        <p className="d_timeline-text"><span className="d_title">Lead Frontend Engineer - Angular<span>Konza Silicon</span></span>
                                        <ul>
                                            <li>Chaired sprint meetings to ensure that tasks were well assigned and completed.</li>
                                            <li>Ensured that the goals of the team were achieved fueled by the contribution of each and every member of the team.</li>
                                            <li>Coordinated with other team leads to ensure that the overall mission and vision of the company are met.</li>
                                        </ul>
                                        </p>
                                    </li>
                                    <li className="d_timeline-item">
                                        <h3 className="d_timeline-title">Jun 2019 - Jan 2022</h3>
                                        <p className="d_timeline-text"><span className="d_title">Senior Frontend Engineer - Angular<span>Konza Silicon</span></span>
                                        <ul>
                                            <li>Worked together with the UI/UX team to ensure that the product designs have been fulfilled to the latter, using Angular and TypeScript, and SCSS for styling the web pages.</li>
                                            <li>Reduced the Angular app build time from 2 hours to ~10 minutes, I achieved this by ensuring reusing of components and reducing redundant code.</li>
                                            <li>Designed and implemented deployment strategy of the frontend application i.e creating pipelines to automate the development/release cycle and containerizing the app using Docker.</li>
                                            <li>Wrote unit tests for components using Jasmine, therefore ensuring that only quality code free from bugs is shipped to production.</li>
                                            <li>Defined the front-end architecture, hence ensuring that the code base is well structured and easy to maintain.</li>
                                            <li>Optimized solutions, and applying suitable algorithms hence ensuring that the application is not expensive to the client in terms of computing resources.</li>
                                            <li>Implemented lazy loading approach for the back office application. The app's performance improved by 50%. I achieved this by breaking the app into smaller lazy-loaded modules.</li>
                                            <li>I orchestrated the containers using Docker Swarm which helped with the high availability of the application.</li>
                                        </ul>
                                        </p>
                                    </li>
                                    <li className="d_timeline-item">
                                        <h3 className="d_timeline-title">Sep 2019 - Jan 2020</h3>
                                        <p className="d_timeline-text"><span className="d_title">React Developer (remote)<span>Better Together Organization</span></span>
                                        <ul>
                                            <li>Consumed Instagram API to display the company's Instagram activity.</li>
                                            <li>Split the application built in React (Next.js) into smaller components, therefore ensuring that the code base is organized and easier to maintain.</li>
                                            <li>Used the Styled Components library in the project, which ensured that the code was much more concise and declarative.</li>
                                        </ul>
                                        </p>
                                    </li>
                                    <li className="d_timeline-item">
                                        <h3 className="d_timeline-title">Jan 2016 - Jun 2018</h3>
                                        <p className="d_timeline-text"><span className="d_title">Junior Web Developer (remote)<span>Safe Meet Organization</span></span>
                                        <ul>
                                            <li>Developed a system to enable students to report bullying cases in the United States of America. This was a mitigation measure for school shootouts as a consequence of bullying, following the Florida School shooting.</li>
                                            <li>Developed a portal to allow students to fill forms for reporting the cases. The cases could be reported anonymously or through an adult on behalf of the student.</li>
                                            <li>Developed a video streaming service that allowed students to upload videos of themselves to the platform.</li>
                                            <li>Developed a mailing service in PHP to enable students to send and receive emails from the platform.</li>
                                        </ul>
                                        </p>
                                    </li>
                                </ul>
                            </div>
                        </div>
                        <div className="spacer-50"></div>
                        <div className="row">
                            <div className="col-md-6">
                                <h3 className="s_border">Programming Skills (confidence)</h3>
                                <div className="skill-bar">
                                    <h5>HTML &amp; CSS</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="95%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>JavaScript</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="98%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>TypeScript</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="98%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>Angular</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="90%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>React</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="90%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>NestJS</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="90%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>Node.js</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="90%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>DevOps</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="75%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>AWS</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="70%"></div>
                                    </div>
                                </div>
                            </div>
                            <div className="col-md-6">
                                <h3 className="s_border">Other Skills (confidence)</h3>
                                <div className="skill-bar">
                                    <h5>UI/UX</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="75%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>Figma</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="75%"></div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
        </>
    );
}
 
export default Resume;