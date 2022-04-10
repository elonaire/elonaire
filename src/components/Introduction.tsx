import { FunctionComponent } from "react";

interface IntroductionProps {
    
}
 
const Introduction: FunctionComponent<IntroductionProps> = () => {
    return (
        <>
        <section id="section-main" className="vertical-center text-light" data-bgimage="url(images/background/2.jpg) top">
                    <div className="container">
                        <div className="row align-items-center">
                            <div className="col-md-6">
                                <h1 className="wow fadeInRight" data-wow-delay=".4s">I'm <span className="id-color">Elon</span>  Aseneka Idiong'o</h1>
                                <p className="lead wow fadeInRight" data-wow-delay=".5s">I am a talented full-stack software engineer, with 6+ years of experience in full-stack development. I have an interest in Game Development and the Internet of Things technology.</p>
                                <div className="spacer-single"></div>
                                <a href="#section-about" className="btn-custom light wow fadeInRight scoll-to" data-wow-delay=".6s">Download CV</a>
                            </div>

                            <div className="col-md-6 sm-hide"><blockquote className="text-light pull-right wow fadeInRight" data-wow-delay=".6s">Software Engineer</blockquote></div>
                        </div>
                    </div>
                </section>
        </>
    );
}
 
export default Introduction;