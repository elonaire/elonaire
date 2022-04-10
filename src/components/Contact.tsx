import { FunctionComponent } from "react";

interface ContactProps {
    
}
 
const Contact: FunctionComponent<ContactProps> = () => {
    return (
        <>
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
        </>
    );
}
 
export default Contact;