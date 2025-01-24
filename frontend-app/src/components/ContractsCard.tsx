"use client";
import { Link } from "react-router";
import DownloadIcon from '../assets/Icon.svg';

import Button from "./Button";
interface ItemProps {
  item : {
    id:string,
    tag : string,
    content:string,
    title : string,
    line : string,
    signed_date : string,
    tokens : string,
    edoc_link : string,
    pdf_link : string
  }
}
export default function ContractsCard({item} : ItemProps) {
  return (
    <>
    <div className="contract-card">
      <div className={`container`}>
        <div className="container-in">
        <div className="col-12">
          <div className="tag">{item.tag}</div>
          <div className="content" dangerouslySetInnerHTML={{ __html: item.content }}></div>
          <div className="name">{item.title}</div>
          <div className="line">{item.line}</div>
          <div className="fs">
            <div className="fl">Signed date : <span>{item.signed_date}</span></div>
            <div className="fr">Tokens : <span>{item.tokens}</span></div>
            <div className="clr"></div>
          </div>
        </div>
        <div className="space-20"></div>
        <div className="clr"></div>
        <div className="col-4 fl col-m-full col-mr-bottom-10">
          <Button text={'SEE PREVIEW'} link={`/contracts/${item.id}`}  active={false} />
        </div>
        <div className="col-3 col-m-half fr">
          <Button icon={DownloadIcon} icononright={true} style="style5" text={'PDF'} link={''}  active={false} />
        </div>
        <div className="col-3 col-m-half fr">
          <Button icon={DownloadIcon} icononright={true} style="style5" text={'EDOC'} link={''}  active={false} />
        </div>
        
        <div className="clr"></div>
      </div>
      </div>
    </div>
    </>
  );
}
