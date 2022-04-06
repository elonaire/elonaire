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
                                            {firstName} Aseneka Idiong'o
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
                                <p className="lead wow fadeInRight" data-wow-delay=".5s">At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident.</p>
                                <div className="spacer-single"></div>
                                <a href="#section-about" className="btn-custom light wow fadeInRight scoll-to" data-wow-delay=".6s">Download CV</a>
                            </div>

                            <div className="col-md-6 sm-hide"><blockquote className="text-light pull-right wow fadeInRight" data-wow-delay=".6s">Be creative and have fun!</blockquote></div>
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
                                        <h3 className="d_timeline-title">Feb 2002 - Mar 2003</h3>
                                        <p className="d_timeline-text"><span className="d_title">Master in Design<span>New York University</span></span>Lorem ipsum dolor sit amet consectetur adipisicing elit. Animi tempora perferendis vero officia enim impedit voluptatem dignissimos, veniam ratione est alias rerum aperiam, nam aliquam reprehenderit iste dolor.</p>
                                    </li>
                                    <li className="d_timeline-item">
                                        <h3 className="d_timeline-title">Feb 2002 - Mar 2003</h3>
                                        <p className="d_timeline-text"><span className="d_title">Bachelor of Arts<span>University of London</span></span>Lorem ipsum dolor sit amet consectetur adipisicing elit. Animi tempora perferendis vero officia enim impedit voluptatem dignissimos, veniam ratione est alias rerum aperiam, nam aliquam reprehenderit iste dolor.</p>
                                    </li>
                                    <li className="d_timeline-item">
                                        <h3 className="d_timeline-title">Feb 2002 - Mar 2003</h3>
                                        <p className="d_timeline-text"><span className="d_title">Artist of College</span>Lorem ipsum dolor sit amet consectetur adipisicing elit. Animi tempora perferendis vero officia enim impedit voluptatem dignissimos, veniam ratione est alias rerum aperiam, nam aliquam reprehenderit iste dolor.</p>
                                    </li>
                                </ul>
                            </div>
                            <div className="col-md-6 wow fadeInRight" data-wow-delay=".5s">
                                <h3 className="s_border">Work Experiences</h3>
                                <ul className="d_timeline">
                                    <li className="d_timeline-item">
                                        <h3 className="d_timeline-title">Feb 2015 - 2019</h3>
                                        <p className="d_timeline-text"><span className="d_title">Lead &amp; UI / UX Designer<span>Apple</span></span>Lorem ipsum dolor sit amet consectetur adipisicing elit. Animi tempora perferendis vero officia enim impedit voluptatem dignissimos, veniam ratione est alias rerum aperiam, nam aliquam reprehenderit iste dolor.</p>
                                    </li>
                                    <li className="d_timeline-item">
                                        <h3 className="d_timeline-title">Feb 2010 - Mar 2015</h3>
                                        <p className="d_timeline-text"><span className="d_title">Senior &amp; UI / UX Designer<span>Google</span></span>Lorem ipsum dolor sit amet consectetur adipisicing elit. Animi tempora perferendis vero officia enim impedit voluptatem dignissimos, veniam ratione est alias rerum aperiam, nam aliquam reprehenderit iste dolor.</p>
                                    </li>
                                    <li className="d_timeline-item">
                                        <h3 className="d_timeline-title">Feb 2006 - Mar 2010</h3>
                                        <p className="d_timeline-text"><span className="d_title">Junior &amp; UI / UX Designer<span>Microsoft</span></span>Lorem ipsum dolor sit amet consectetur adipisicing elit. Animi tempora perferendis vero officia enim impedit voluptatem dignissimos, veniam ratione est alias rerum aperiam, nam aliquam reprehenderit iste dolor.</p>
                                    </li>
                                </ul>
                            </div>
                        </div>
                        <div className="spacer-50"></div>
                        <div className="row">
                            <div className="col-md-6">
                                <h3 className="s_border">Programming Skills</h3>
                                <div className="skill-bar">
                                    <h5>HTML &amp; CSS</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="95%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>Javascript</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="75%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>PHP</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="85%"></div>
                                    </div>
                                </div>
                            </div>
                            <div className="col-md-6">
                                <h3 className="s_border">Other Skills</h3>
                                <div className="skill-bar">
                                    <h5>Copywritting</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="75%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>Drawing</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="90%"></div>
                                    </div>
                                </div>
                                <div className="skill-bar">
                                    <h5>Singing</h5>
                                    <div className="de-progress">
                                        <div className="value"></div>
                                        <div className="progress-bar" data-value="80%"></div>
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
                                <h4 className="title">Who I Am</h4>
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
                                <h2>Hello, I'm <span className="id-color">Amalia Jördis</span></h2>
                                <p>
                                    At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident,
                                    similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Veniam dolore qui sunt dolor anim est proident culpa.
                                </p>
                                <ul className="info-list text-white">
                                    <li><span className="d_title">Age</span><span className="d_value">28</span></li>
                                    <li><span className="d_title">Residence</span><span className="d_value">USA</span></li>
                                    <li><span className="d_title">Address</span><span className="d_value">08 W 36th St, New York, NY 10001</span></li>
                                    <li><span className="d_title">Email</span><span className="d_value">contactme@envato.com</span></li>
                                    <li><span className="d_title">Phone</span><span className="d_value">+1 333 1224 6779</span></li>
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
                                        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt.</p>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4 col-md-6 mb30 wow fadeInRight" data-wow-delay=".25s">
                                <div className="f-box f-icon-left f-icon-box">
                                    <i className="fa fa-paper-plane-o id-color"></i>
                                    <div className="fb-text">
                                        <h4>Email Marketing</h4>
                                        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt.</p>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4 col-md-6 mb-30 wow fadeInRight" data-wow-delay=".5s">
                                <div className="f-box f-icon-left f-icon-box">
                                    <i className="fa fa-line-chart id-color"></i>
                                    <div className="fb-text">
                                        <h4>Digital Analytics</h4>
                                        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt.</p>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4 col-md-6 mb-sm-30 wow fadeInRight" data-wow-delay="0s">
                                <div className="f-box f-icon-left f-icon-box">
                                    <i className="fa fa-object-group id-color"></i>
                                    <div className="fb-text">
                                        <h4>Visual Editor</h4>
                                        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt.</p>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4 col-md-6 mb-sm-30 wow fadeInRight" data-wow-delay=".25s">
                                <div className="f-box f-icon-left f-icon-box">
                                    <i className="fa fa-hdd-o id-color"></i>
                                    <div className="fb-text">
                                        <h4>Backup &amp; Security</h4>
                                        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt.</p>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4 col-md-6 mb-sm-30 wow fadeInRight" data-wow-delay=".5s">
                                <div className="f-box f-icon-left f-icon-box">
                                    <i className="fa fa-comments-o id-color"></i>
                                    <div className="fb-text">
                                        <h4>IT Consultancy</h4>
                                        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt.</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div className="spacer-80"></div>
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
                        </div>
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
