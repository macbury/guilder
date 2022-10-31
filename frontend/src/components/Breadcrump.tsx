import React from 'react';
import { Link } from 'react-router-dom';
import Breadcrumb from 'rsuite/Breadcrumb';

export interface INavLinkProps {
  href: string,
  as: any
}

const NavLink = React.forwardRef<HTMLAnchorElement, INavLinkProps>((props, ref) => {
  const { href, as, ...rest } = props;
  return (
    <Link ref={ref} to={href} as={as} {...rest} />
  );
});

export default Breadcrumb

export function BreadcrumbItem(props) {
  return <Breadcrumb.Item as={NavLink} {...props} />
}
