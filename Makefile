PORTNAME=	netavark
PORTVERSION=	1.17.0
DISTVERSIONPREFIX=	v
CATEGORIES=	sysutils

MAINTAINER=	osa@FreeBSD.org
COMMENT=	Container network stack
WWW=		https://github.com/containers/netavark

LICENSE=	APACHE20
LICENSE_FILE=	${WRKSRC}/LICENSE

USES=		cargo
#LIB_DEPENDS=	libzstd.so:archivers/zstd

CARGO_BUILDDEP=	no
#MAKE_ENV=	RUSTFLAGS="${RUSTFLAGS} -D__BSD_VISIBLE -D_KERNEL"

USE_GITHUB=	yes
GH_ACCOUNT=	containers

.include "${.CURDIR}/Makefile.crates"

.include <bsd.port.mk>
