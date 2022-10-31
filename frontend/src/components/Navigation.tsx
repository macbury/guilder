import React from 'react';
import Nav from 'rsuite/Nav';
import { Link, useMatch } from 'react-router-dom';
import { MdOutlineAccountBalance, MdOutlineAccountBalanceWallet, MdDashboard, MdLocalActivity, MdOutlineCategory, MdOutlineFavorite, MdOutlineStorefront, MdCloudDownload } from "react-icons/md";
import OffIcon from '@rsuite/icons/Off';
import { useAuthenticationManager } from '../store/hooks/session';

const NavLink = React.forwardRef((props : any, ref : any) => {
  const { to, ...rest } = props;
  return (
    <Link to={to} ref={ref} {...rest} />
  );
});

function NavItem({ children, to, icon } : any) {
  const active = !!useMatch(`${to}/*`);

  return (
    <Nav.Item as={NavLink} active={active} to={to} icon={icon}>
      {children}
    </Nav.Item>
  )
}

function NavIcon({ IconClass }) {
  return (
    <span className="rs-icon">
      <IconClass />
    </span>
  )
}

export default function Navigation() {
  const { logout } = useAuthenticationManager();

  return (
    <React.Fragment>
      <Nav>
        <NavItem to="/" icon={<NavIcon IconClass={MdDashboard} />}>
          Home
        </NavItem>
        <NavItem to="/bonds" icon={<NavIcon IconClass={MdLocalActivity} />}>
          Bonds
        </NavItem>
        <NavItem to="/assets" icon={<NavIcon IconClass={MdOutlineFavorite} />}>
          Assets
        </NavItem>
        <NavItem to="/holdings" icon={<NavIcon IconClass={MdOutlineStorefront} />}>
          Holdings
        </NavItem>
        <NavItem to="/wallets" icon={<NavIcon IconClass={MdOutlineAccountBalanceWallet} />}>
          Wallets
        </NavItem>
        <NavItem to="/accounts" icon={<NavIcon IconClass={MdOutlineAccountBalance} />}>
          Accounts
        </NavItem>
        <NavItem to="/categories" icon={<NavIcon IconClass={MdOutlineCategory} />}>
          Categories
        </NavItem>
        <NavItem to="/integrations" icon={<NavIcon IconClass={MdCloudDownload} />}>
          Integrations
        </NavItem>
      </Nav>
      <Nav>
        <Nav.Item icon={<NavIcon IconClass={OffIcon} />} onClick={logout}>Logout</Nav.Item>
      </Nav>
    </React.Fragment>
  )
}
