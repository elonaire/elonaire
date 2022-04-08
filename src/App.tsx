import React, { useState } from 'react';
import './App.css';

function App() {
    const [firstName] = useState('Elon')
    return (
        <div id="wrapper">
            <div id="btn-exit">
                <div className="line-1"></div>
                <div className="line-2"></div>
            </div>
            <div className="transition"></div>
            {/* header begin */}
            <header className="transparent">
                <div className="container">
                    <div className="row">
                        <div className="col-md-12">
                            <div className="d-flex justify-content-between">
                                <div className="align-self-center header-col-left">
                                    {/* logo begin */}
                                    <div id="logo">
                                        <a href="index.html">
                                            <img className='main-logo' src='images/logo.png' />
                                        </a>
                                    </div>
                                    {/* logo close */}
                                </div>
                                <div className="align-self-center ml-auto header-col-mid">
                                    {/* mainmenu begin */}
                                    <ul id="mainmenu" className="scrollnav">
                                        <li><a href="#section-resume">Resume</a></li>
                                        <li></li>
                                        <li><a href="#section-about">About Me</a></li>
                                        <li></li>
                                        <li><a href="#section-portfolio">Portfolio</a></li>
                                        <li></li>
                                        <li><a href="#section-blog">Blog</a></li>
                                        <li></li>
                                        <li><a href="#section-contact">Contact</a></li>
                                        <li></li>
                                    </ul>
                                    {/* mainmenu close */}
                                </div>
                                <div className="align-self-center ml-auto header-col-right">
                                    <div className="social-icons s-border sm-hide">
                                        <a href="#"><i className="fa fa-facebook fa-lg"></i></a>
                                        <a href="#"><i className="fa fa-twitter fa-lg"></i></a>
                                        <a href="#"><i className="fa fa-linkedin fa-lg"></i></a>
                                        <a href="#"><i className="fa fa-instagram fa-lg"></i></a>
                                    </div>
                                    <span id="menu-btn"></span>
                                </div>
                                <div className="clearfix"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </header>
            {/* header close */}

            {/* content begin */}
            <div className="no-bottom no-top dark">
                <div id="top"></div>
                {/* section begin */}
                <section id="section-main" className="vertical-center text-light" data-bgimage="url(images/background/2.jpg) top">
                    <div className="container">
                        <div className="row align-items-center">
                            <div className="col-md-6">
                                <h1 className="wow fadeInRight" data-wow-delay=".4s">I'm <span className="id-color">{firstName}</span>  Aseneka Idiong'o</h1>
                                <p className="lead wow fadeInRight" data-wow-delay=".5s">I am a talented full-stack software engineer, with 6+ years of experience in full-stack development. I have an interest in Game Development and the Internet of Things technology.</p>
                                <div className="spacer-single"></div>
                                <a href="#section-about" className="btn-custom light wow fadeInRight scoll-to" data-wow-delay=".6s">Download CV</a>
                            </div>

                            <div className="col-md-6 sm-hide"><blockquote className="text-light pull-right wow fadeInRight" data-wow-delay=".6s">Software Engineer</blockquote></div>
                        </div>
                    </div>
                </section>
                {/* section close */}

                {/* section begin */}
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
                {/* section close */}

                {/* section begin */}
                <section id="section-about" className="dark">
                    <div className="container">
                        <div className="row">
                            <div className="col-lg-12 wow fadeInRight">
                                <h4 className="title">Who Am I?</h4>
                                <h2 className="title">About Me</h2>
                                <div className="title-underline"></div>
                            </div>
                        </div>
                        <div className="row">
                            <div className="col-lg-6 mb-sm-30 text-center">
                                <div className="de-images">
                                    <div className="d-hover-zoom wow fadeInRight">
                                        <a className="image-popup" href="images/misc/1.jpg">
                                            <img className="img-fluid" src="images/misc/1.jpg" alt="" />
                                        </a>
                                    </div>
                                    <div className="d-hover-zoom di-small-2 wow fadeInLeft">
                                        <a className="image-popup" href="images/misc/2.jpg">
                                            <img src="images/misc/2.jpg" alt="" />
                                        </a>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-5 offset-md-1 wow fadeInRight" data-wow-delay=".5s">
                                <h2>Hello, I'm <span className="id-color">Elon Aseneka Idiong'o</span></h2>
                                <p>
                                I am a talented full-stack software engineer and I have 6+ years of experience in building robust small and enterprise applications. I have built various web applications using Node.js, NestJS, Angular, and React. I have built desktop applications using ElectronJS and mobile applications using React Native. Besides that, I use Figma for application designing and prototyping. I have an interest in Game Development and the Internet of Things.
                                </p>
                                <ul className="info-list text-white">
                                    <li><span className="d_title">Age</span><span className="d_value">26</span></li>
                                    <li><span className="d_title">Residence</span><span className="d_value">Kenya</span></li>
                                    <li><span className="d_title">Address</span><span className="d_value">Unity Homes, Unity West, Tatu City</span></li>
                                    <li><span className="d_title">Email</span><span className="d_value">info@techietenka.com</span></li>
                                    <li><span className="d_title">Phone</span><span className="d_value">+254(0) 704730039</span></li>
                                </ul>
                            </div>
                        </div>
                        <div className="spacer-80"></div>
                        <div className="row">
                            <div className="col-md-12">
                                <h3 className="s_border">My Services</h3>
                            </div>
                            <div className="col-lg-4 col-md-6 mb30">
                                <div className="f-box f-icon-left f-icon-box">
                                    <i className="fa fa-laptop id-color"></i>
                                    <div className="fb-text">
                                        <h4>Website Design</h4>
                                        <p>I have UI/UX skills. I am skilled in Figma and I am able to create wireframes, high fidelity mockups and prototypes.</p>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4 col-md-6 mb30 wow fadeInRight" data-wow-delay=".25s">
                                <div className="f-box f-icon-left f-icon-box">
                                    <i className="fa fa-globe id-color"></i>
                                    <div className="fb-text">
                                        <h4>Responsive Web Applications</h4>
                                        <p>I am able to apply the mobile-first principle to create web applications that display accurately across different viewports.</p>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4 col-md-6 mb-30 wow fadeInRight" data-wow-delay=".5s">
                                <div className="f-box f-icon-left f-icon-box">
                                    <i className="fa fa-line-chart id-color"></i>
                                    <div className="fb-text">
                                        <h4>Data Visualization</h4>
                                        <p>With the help of visualization tools such as D3.js and other chart libraries, I am able to help users gain insights into their data.</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                        {/* <div className="spacer-80"></div>
                        <div className="row">
                            <div className="col-md-12">
                                <h3 className="s_border">Testimonials</h3>
                            </div>
                            <div className="col-md-12">
                                <div id="testimonial-carousel" className="owl-carousel owl-theme wow fadeInUp">
                                    <blockquote className="testimonial-big text-light">
                                        I'm always impressed with the services. Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
                                        nostrud exercitation ullamco laboris nisi ut aliquip.
                                        <img src="images/people/1.jpg" alt="" />
                                        <span className="name">John, Customer</span>
                                    </blockquote>
                                    <blockquote className="testimonial-big text-light">
                                        I'm always impressed with the services. Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
                                        nostrud exercitation ullamco laboris nisi ut aliquip.
                                        <img src="images/people/2.jpg" alt="" />
                                        <span className="name">Sandra, Customer</span>
                                    </blockquote>
                                    <blockquote className="testimonial-big text-light">
                                        I'm always impressed with the services. Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
                                        nostrud exercitation ullamco laboris nisi ut aliquip.
                                        <img src="images/people/3.jpg" alt="" />
                                        <span className="name">Michael, Customer</span>
                                    </blockquote>
                                    <blockquote className="testimonial-big text-light">
                                        I'm always impressed with the services. Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
                                        nostrud exercitation ullamco laboris nisi ut aliquip.
                                        <img src="images/people/4.jpg" alt="" />
                                        <span className="name">George, Customer</span>
                                    </blockquote>
                                </div>
                            </div>
                        </div> */}
                    </div>
                </section>
                {/* section close */}

                {/* section begin */}
                <section id="section-portfolio" className="dark">
                    <div className="container">
                        {/* portfolio filter begin */}
                        <div className="row">
                            <div className="col-lg-12 wow fadeInRight">
                                <h4 className="title">Showcase of my best works</h4>
                                <h2 className="title">My Portfolio</h2>
                                <div className="title-underline"></div>
                            </div>
                            <div className="col-md-12 text-center wow fadeInRight" data-wow-delay=".25s">
                                <ul id="filters">
                                    <li><a href="#" data-filter="*" id="btn_all" className="selected">all projects</a></li>
                                    <li><a href="#" data-filter=".illustration">illustration</a></li>
                                    <li><a href="#" data-filter=".mobile">mobile</a></li>
                                    <li><a href="#" data-filter=".photography">photography</a></li>
                                    <li><a href="#" data-filter=".website">website</a></li>
                                </ul>
                                <div id="gallery" className="gallery full-gallery de-gallery zoom-gallery row sequence wow fadeInRight" data-wow-delay=".5s">
                                    {/* gallery item */}
                                    <div className="item col-md-4 mb30 gallery-item illustration website" data-value="project-details-image.html">
                                        <a className="picframe" href="images/portfolio/1.jpg" title="Exhibiz" data-source="https://themeforest.net/item/exhibiz-event-conference-and-meetup/28663470">
                                            <span className="overlay">
                                                <span className="d-title">Exhibiz</span>
                                            </span>
                                            <img src="images/portfolio/1.jpg" alt="" />
                                        </a>
                                    </div>
                                    {/* close gallery item */}
                                    {/* gallery item */}
                                    <div className="item col-md-4 mb30 gallery-item photography">
                                        <a className="picframe" href="images/portfolio/2.jpg" title="Bolo" data-source="https://themeforest.net/item/bolo-onepage-creative-website-template/25030305">
                                            <span className="overlay">
                                                <span className="d-title">Bolo</span>
                                            </span>
                                            <img src="images/portfolio/2.jpg" alt="" />
                                        </a>
                                    </div>
                                    {/* close gallery item */}
                                    {/* gallery item */}
                                    <div className="item col-md-4 mb30 gallery-item illustration">
                                        <a className="picframe" href="images/portfolio/3.jpg" title="Elaxo" data-source="https://themeforest.net/item/elaxo-app-and-software-website-template/29226060">
                                            <span className="overlay">
                                                <span className="d-title">Elaxo</span>
                                            </span>
                                            <img src="images/portfolio/3.jpg" alt="" />
                                        </a>
                                    </div>
                                    {/* close gallery item */}
                                    {/* gallery item */}
                                    <div className="item col-md-4 mb30 gallery-item photography illustration mobile">
                                        <a className="picframe" href="images/portfolio/4.jpg" title="Jonna" data-source="https://themeforest.net/item/jonna-personal-portfolio-website-template/26361888">
                                            <span className="overlay">
                                                <span className="d-title">Jonna</span>
                                            </span>
                                            <img src="images/portfolio/4.jpg" alt="" />
                                        </a>
                                    </div>
                                    {/* close gallery item */}
                                    {/* gallery item */}
                                    <div className="item col-md-4 mb30 gallery-item photography mobile website">
                                        <a className="picframe" href="images/portfolio/5.jpg" title="Justica" data-source="https://themeforest.net/item/justica-lawyer-and-attorney-website-template/29485331">
                                            <span className="overlay">
                                                <span className="d-title">Justica</span>
                                            </span>
                                            <img src="images/portfolio/5.jpg" alt="" />
                                        </a>
                                    </div>
                                    {/* close gallery item */}
                                    {/* gallery item */}
                                    <div className="item col-md-4 mb30 gallery-item mobile website">
                                        <a className="picframe" href="images/portfolio/6.jpg" title="Bluetec" data-source="https://themeforest.net/item/bluetec-saas-it-software-startup-landing-page-template/27106031">
                                            <span className="overlay">
                                                <span className="d-title">Bluetec</span>
                                            </span>
                                            <img src="images/portfolio/6.jpg" alt="" />
                                        </a>
                                    </div>
                                    {/* close gallery item */}
                                </div>
                            </div>
                            {/* portfolio filter close */}
                        </div>
                    </div>
                </section>
                {/* section close */}

                {/* section begin */}
                <section id="section-blog" className="dark">
                    <div className="container">
                        <div className="row">
                            <div className="col-lg-12 wow fadeInRight">
                                <h4 className="title">This is my story</h4>
                                <h2 className="title">My Blog</h2>
                                <div className="title-underline"></div>
                            </div>
                        </div>
                        <div className="row wow fadeInRight" data-wow-delay=".25s">
                            <div className="col-lg-4 col-md-6 mb30">
                                <div className="bloglist item">
                                    <div className="post-content">
                                        <div className="post-image d-hover-zoom">
                                            <a className="image-popup" href="images/news/1.jpg">
                                                <img alt="" src="images/news/1.jpg" />
                                            </a>
                                            <div className="post-info">
                                                <div className="inner">
                                                    <span className="post-date">10 Dec 2020</span>
                                                </div>
                                            </div>
                                        </div>
                                        <div className="post-text">
                                            <h4><a href="blog-single.html">Better User Interface</a></h4>
                                            <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4 col-md-6 mb30">
                                <div className="bloglist item">
                                    <div className="post-content">
                                        <div className="post-image d-hover-zoom">
                                            <a className="image-popup" href="images/news/2.jpg">
                                                <img alt="" src="images/news/2.jpg" />
                                            </a>
                                            <div className="post-info">
                                                <div className="inner">
                                                    <span className="post-date">10 Dec 2020</span>
                                                </div>
                                            </div>
                                        </div>
                                        <div className="post-text">
                                            <h4><a href="blog-single.html">Experts Web Design Tips</a></h4>
                                            <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4 col-md-6 mb30">
                                <div className="bloglist item">
                                    <div className="post-content">
                                        <div className="post-image d-hover-zoom">
                                            <a className="image-popup" href="images/news/3.jpg">
                                                <img alt="" src="images/news/3.jpg" />
                                            </a>
                                            <div className="post-info">
                                                <div className="inner">
                                                    <span className="post-date">10 Dec 2020</span>
                                                </div>
                                            </div>
                                        </div>
                                        <div className="post-text">
                                            <h4><a href="blog-single.html">Importance Of Web Design</a></h4>
                                            <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4 col-md-6 mb30">
                                <div className="bloglist item">
                                    <div className="post-content">
                                        <div className="post-image d-hover-zoom">
                                            <a className="image-popup" href="images/news/4.jpg">
                                                <img alt="" src="images/news/4.jpg" />
                                            </a>
                                            <div className="post-info">
                                                <div className="inner">
                                                    <span className="post-date">10 Dec 2020</span>
                                                </div>
                                            </div>
                                        </div>
                                        <div className="post-text">
                                            <h4><a href="blog-single.html">Avoid These Erros In UI Design</a></h4>
                                            <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4 col-md-6 mb30">
                                <div className="bloglist item">
                                    <div className="post-content">
                                        <div className="post-image d-hover-zoom">
                                            <a className="image-popup" href="images/news/5.jpg">
                                                <img alt="" src="images/news/5.jpg" />
                                            </a>
                                            <div className="post-info">
                                                <div className="inner">
                                                    <span className="post-date">10 Dec 2020</span>
                                                </div>
                                            </div>
                                        </div>
                                        <div className="post-text">
                                            <h4><a href="blog-single.html">Make Mobile Website Faster</a></h4>
                                            <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4 col-md-6 mb30">
                                <div className="bloglist item">
                                    <div className="post-content">
                                        <div className="post-image d-hover-zoom">
                                            <a className="image-popup" href="images/news/6.jpg">
                                                <img alt="" src="images/news/6.jpg" />
                                            </a>
                                            <div className="post-info">
                                                <div className="inner">
                                                    <span className="post-date">10 Dec 2020</span>
                                                </div>
                                            </div>
                                        </div>
                                        <div className="post-text">
                                            <h4><a href="blog-single.html">How Sell Digital Product</a></h4>
                                            <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
                {/* section close */}

                {/* section begin */}
                <section id="section-contact" className="dark">
                    <div className="container">
                        <div className="row">
                            <div className="col-lg-12 wow fadeInRight">
                                <h4 className="title">I'm available for hire</h4>
                                <h2 className="title">Contact Me</h2>
                                <div className="title-underline"></div>
                            </div>
                        </div>
                        <form name="contactForm" id="contact_form" className="row form-default wow fadeInRight" data-wow-delay=".25s" method="post" action="email.php">
                            <div className="col-md-6">
                                <div className="field-set">
                                    <input type="text" name="name" id="name" className="form-control" placeholder="Your Name" />
                                    <div className="line-fx"></div>
                                </div>
                                <div className="field-set">
                                    <input type="text" name="email" id="email" className="form-control" placeholder="Your Email" />
                                    <div className="line-fx"></div>
                                </div>
                                <div className="field-set">
                                    <input type="text" name="phone" id="phone" className="form-control" placeholder="Your Phone" />
                                    <div className="line-fx"></div>
                                </div>
                            </div>
                            <div className="col-md-6">
                                <div className="field-set">
                                    <textarea name="message" id="message" className="form-control" placeholder="Your Message"></textarea>
                                    <div className="line-fx"></div>
                                </div>
                            </div>
                            <div className="spacer-single"></div>
                            <div className="col-md-12 text-center">
                                <div id="submit">
                                    <input type="submit" id="send_message" value="Send" className="btn btn-custom color-2" />
                                </div>
                                <div id="mail_success" className="success">Your message has been sent successfully.</div>
                                <div id="mail_fail" className="error">Sorry, error occured this time sending your message.</div>
                            </div>
                        </form>
                    </div>
                </section>
            </div>
            {/* content close */}
        </div>
    );
}

export default App;
