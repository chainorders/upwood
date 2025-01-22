import logoImage from '../assets/logo.svg';
import menuIcon from '../assets/menu.svg';
import closeMenuIcon from '../assets/close-menu.svg';
import logout from '../assets/logout.svg';
import { Link } from 'react-router';

export interface NavItem {
    name: string;
    url: string;
    iconNormal: string;
    iconActive: string;
    isActive: boolean;
}

function VerticalNav(props: { closeMenu: () => void, navItems: NavItem[] }) {
    return (
        <div className='verticle-navigation'>
            <ul>
                {props.navItems.map((item, index) => (
                    <li key={index} className={item.isActive ? 'active' : ''}
                        onClick={props.closeMenu}>
                        <img
                            className='navigation-icon'
                            src={item.isActive ? item.iconActive : item.iconNormal}
                            alt=""
                            width={24}
                            height={24}
                        />
                        <Link to={item.url}>{item.name}</Link>
                    </li>
                ))}
            </ul>
        </div>
    );
}

export default function Header(props: { navItems: NavItem[], logout: () => void }) {
    const openMenu = () => {
        document.body.classList.toggle("menuopen");
    };

    const closeMenu = () => {
        document.body.classList.remove("menuopen");
    };

    return (
        <>
            <div className="header">
                <div className="top">
                    <div className="logo fl">
                        <img
                            src={logoImage}
                            alt=""
                            width={176}
                            height={38}
                        />
                    </div>
                    <img
                        src={menuIcon}
                        width={40}
                        height={40}
                        className="menuopen-bar fr" onClick={openMenu}
                    />
                    <img
                        src={closeMenuIcon}
                        width={40}
                        height={40}
                        className="menuclose-bar fr" onClick={openMenu}
                    />
                    <div className="clr"></div>
                </div>

                <span className="logout" onClick={props.logout}>
                    <img
                        className='icon'
                        src={logout}
                        alt=""
                        width={24}
                        height={24}
                    />
                    <span className="text" onClick={() => {
                        console.log('logout');
                        props.logout();
                    }}>Logout</span>
                </span>
            </div>
            <VerticalNav closeMenu={closeMenu} navItems={props.navItems} />
        </>
    );
}
