import { FunctionComponent } from "react";

interface PortfolioProps {
    
}
 
const Portfolio: FunctionComponent<PortfolioProps> = () => {
    return (
        <>
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
        </>
    );
}
 
export default Portfolio;