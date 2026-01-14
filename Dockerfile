FROM scratch
ARG TARGET_PATH

COPY ./${TARGET_PATH}/rustpl /rustpl

ENTRYPOINT [ "/rustpl" ]
