import React, { useState } from 'react';
import './App.css';
import About from './components/About';
import Blog from './components/Blog';
import Contact from './components/Contact';
import Introduction from './components/Introduction';
import NavigationBar from './components/NavigationBar';
import Portfolio from './components/Portfolio';
import Resume from './components/Resume';

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
            <NavigationBar />
            {/* header close */}

            {/* content begin */}
            <div className="no-bottom no-top dark">
                <div id="top"></div>
                {/* section begin */}
                <Introduction />
                {/* section close */}

                {/* section begin */}
                <Resume />
                {/* section close */}

                {/* section begin */}
                <About />
                {/* section close */}

                {/* section begin */}
                <Portfolio />
                {/* section close */}

                {/* section begin */}
                <Blog />
                {/* section close */}

                {/* section begin */}
                <Contact />
                {/* section close */}
            </div>
            {/* content close */}
        </div>
    );
}

export default App;
