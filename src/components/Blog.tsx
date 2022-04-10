import { FunctionComponent } from "react";

interface BlogProps {
    
}
 
const Blog: FunctionComponent<BlogProps> = () => {
    return (
        <>
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
        </>
    );
}
 
export default Blog;