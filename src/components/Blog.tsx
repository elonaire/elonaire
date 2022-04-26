import { FunctionComponent, useEffect, useState } from "react";
import Axios from 'axios';

interface BlogProps {
    
}
 
const Blog: FunctionComponent<BlogProps> = () => {
    const [posts, setPosts] = useState([]);

    const fetchPosts = async () => {
        try {
          const res = await Axios.get(
            `https://www.googleapis.com/blogger/v3/blogs/${process.env.REACT_APP_BLOGGER_BLOG_ID}/posts?key=${process.env.REACT_APP_BLOGGER_API_KEY}&fetchImages=true`
          );
    
          console.log(res.data);
          
          setPosts(res.data.items);
        //   setPosts(res.data);
        } catch (error) {
          console.log(error);
        }
      };
    
      useEffect(() => {
        fetchPosts();
      }, []);
    
    return (
        <>
        <section id="section-blog" className="dark">
                    <div className="container">
                        <div className="row">
                            <div className="col-lg-12 wow fadeInRight">
                                <h4 className="title">These are my articles</h4>
                                <h2 className="title">My Blog</h2>
                                <div className="title-underline"></div>
                            </div>
                        </div>
                        <div className="row wow fadeInRight" data-wow-delay=".25s">
                            {posts.map((post: any) => <div key={post.id} className="col-lg-4 col-md-6 mb30">
                                <div className="bloglist item">
                                    <div className="post-content">
                                        <div className="post-image d-hover-zoom">
                                            <a className="image-popup" href={post?.url} rel="noreferrer" target="_blank">
                                                <img alt="" src={post?.images[0].url} />
                                            </a>
                                            <div className="post-info">
                                                <div className="inner">
                                                    <span className="post-date">{`${new Date(post?.published)}`}</span>
                                                </div>
                                            </div>
                                        </div>
                                        <div className="post-text">
                                            <h4><a href={post?.url} rel="noreferrer" target="_blank">{post?.title}</a></h4>
                                            {/* <p>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p> */}
                                        </div>
                                    </div>
                                </div>
                            </div>)}
                        </div>
                    </div>
                </section>
        </>
    );
}
 
export default Blog;