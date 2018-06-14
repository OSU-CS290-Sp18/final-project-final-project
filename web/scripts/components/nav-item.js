import React from 'react'
import { NavLink } from 'react-router-dom'

const NavItem = ({ path, name }) => (
  <li className="nav-item">
    <NavLink to={path} className="nav-link" activeClassName="nav-active">
      {name}
    </NavLink>
  </li>
)

export default NavItem
