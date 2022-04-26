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
                                    <li><a href="#" data-filter=".react">React</a></li>
                                    <li><a href="#" data-filter=".node">Node.js/NestJS</a></li>
                                    <li><a href="#" data-filter=".angular">Angular</a></li>
                                    <li><a href="#" data-filter=".reactnative">React Native</a></li>
                                </ul>
                                <div id="gallery" className="gallery full-gallery de-gallery zoom-gallery row sequence wow fadeInRight" data-wow-delay=".5s">
                                    {/* gallery item */}
                                    <div className="item col-md-4 mb30 gallery-item angular" data-value="project-details-image.html">
                                        <a className="picframe" href="images/portfolio/1.jpg" title="Ardhisasa" data-source="https://ardhisasa.lands.go.ke">
                                            <span className="overlay">
                                                <span className="d-title">Ardhisasa</span>
                                            </span>
                                            <img src="images/portfolio/1.jpg" alt="portfolio-pic" />
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