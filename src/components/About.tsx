import { FunctionComponent } from "react";

interface AboutProps {
    
}
 
const About: FunctionComponent<AboutProps> = () => {
    return (
        <>
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
                                            <img className="img-fluid" src="images/misc/1.jpg" alt="self-pic" />
                                        </a>
                                    </div>
                                    <div className="d-hover-zoom di-small-2 wow fadeInLeft">
                                        <a className="image-popup" href="images/misc/2.jpg">
                                            <img src="images/misc/2.jpg" alt="self-pic" />
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
        </>
    );
}
 
export default About;