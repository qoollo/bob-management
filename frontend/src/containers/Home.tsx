import reactLogo from '../images/logo.svg';
import rustLogo from '../images/logo2.svg';
import plus from '../images/plus.svg';

export const Home = () => {
    return (
        <div>
            <div style={{ display: 'flex', justifyContent: 'center' }}>
                <img src={rustLogo} className="logo" alt="rust-logo" />
                <img src={plus} alt="plus" />
                <img src={reactLogo} className="logo" alt="react-logo" />
            </div>
            <p>
                Edit <code>frontend/src/App.tsx</code> and save to reload.
            </p>
        </div>
    );
};
