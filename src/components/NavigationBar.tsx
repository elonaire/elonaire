import { FunctionComponent } from "react";

interface NavigationBarProps {
    
}
 
const NavigationBar: FunctionComponent<NavigationBarProps> = () => {
    return (
        <>
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
                                        <a href="https://www.facebook.com/elonaire/" rel="noreferrer" target="_blank"><i className="fa fa-facebook fa-lg"></i></a>
                                        <a href="https://twitter.com/elonaire" rel="noreferrer" target="_blank"><i className="fa fa-twitter fa-lg"></i></a>
                                        <a href="https://www.linkedin.com/in/elon-aseneka-elonaire/" rel="noreferrer" target="_blank"><i className="fa fa-linkedin fa-lg"></i></a>
                                        <a href="https://www.instagram.com/elonaire95/" rel="noreferrer" target="_blank"><i className="fa fa-instagram fa-lg"></i></a>
                                    </div>
                                    <span id="menu-btn"></span>
                                </div>
                                <div className="clearfix"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </header>
        </>
    );
}
 
export default NavigationBar;