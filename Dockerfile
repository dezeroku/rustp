FROM python:3.9 as z3
RUN mkdir -p /z3_built
COPY z3 /z3
WORKDIR /z3
RUN python3 scripts/mk_make.py --prefix=/z3_built
WORKDIR /z3/build
RUN make -j${nproc}
RUN make install

FROM ubuntu:20.04 as temp
COPY --from=z3 /z3_built/bin /usr/bin
COPY --from=z3 /z3_built/lib /usr/lib
COPY --from=z3 /z3_built/include /usr/include