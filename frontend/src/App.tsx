import './App.css';
import { Home } from './containers/Home';
import { Route, useNavigate, Routes } from 'react-router-dom';

function App() {
    const navigate = useNavigate();

    // @ts-ignore
    return (
        <div className="App">
            <div className="App-nav-header">
                <div style={{ display: 'flex', flex: 1 }}>
                    <a className="NavButton" onClick={() => navigate('/')}>
                        Home
                    </a>
                </div>
            </div>
            <div style={{ margin: '0 auto', maxWidth: '800px' }}>
                <Routes>
                    <Route path="/" element={<Home />} />
                </Routes>
            </div>
        </div>
    );
}

export default App;
